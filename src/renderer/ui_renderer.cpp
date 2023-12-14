#include "ui_renderer.h"
#include "animation_handler.h"
#include "constant.h"
#include "gear.h"
#include "helpers.h"
#include "key_path.h"
#include "keycodes.h"

#include "lib.rs.h"

namespace ui
{
	static std::map<animation_type, std::vector<TextureData>> animation_frame_map = {};
	static std::vector<std::pair<animation_type, std::unique_ptr<Animation>>> animation_list;

	static std::map<uint8_t, float> cycle_timers = {};

	static std::map<uint32_t, TextureData> key_struct;
	static std::map<uint32_t, TextureData> default_key_struct;
	static std::map<uint32_t, TextureData> PS5_BUTTON_MAP;
	static std::map<uint32_t, TextureData> XBOX_BUTTON_MAP;
	static std::map<std::string, TextureData> ICON_MAP;
	static std::map<std::string, TextureData> HUD_IMAGES_MAP;

	static const float FADEOUT_HYSTERESIS = 0.5f;  // seconds
	static const uint32_t MAX_ICON_DIM    = 300;   // rasterized at 96 dpi
	static constexpr ImVec2 FLAT_UVS[4]   = { ImVec2(0.0f, 0.0f),
		  ImVec2(1.0f, 0.0f),
		  ImVec2(1.0f, 1.0f),
		  ImVec2(0.0f, 1.0f) };


	auto gHudAlpha          = 0.0f;  // this is the current alpha
	auto gGoalAlpha         = 1.0f;  // our goal if we're fading
	auto gMaxAlpha          = 1.0f;  // the least transparent we allow ourselves to be (user setting)
	auto doFadeIn           = true;
	auto gFullFadeDuration  = 3.0f;  // seconds
	auto gFadeDurRemaining  = 2.0f;  // seconds
	auto gIsFading          = false;
	auto delayBeforeFadeout = 0.33f;  // seconds
	bool gDoingBriefPeek    = false;

	// ID3D11BlendState* gBlendState = nullptr;

	ImFont* imFont;
	auto triedFontLoad = false;

	LRESULT ui_renderer::wnd_proc_hook::thunk(const HWND h_wnd,
		const UINT u_msg,
		const WPARAM w_param,
		const LPARAM l_param)
	{
		auto& io = ImGui::GetIO();
		if (u_msg == WM_KILLFOCUS)
		{
			io.ClearInputCharacters();
			io.ClearInputKeys();
		}

		return func(h_wnd, u_msg, w_param, l_param);
	}

	void ui_renderer::d_3d_init_hook::thunk()
	{
		func();

		rlog::info("D3DInit hooked so we can give imgui something to render to."sv);
		const auto renderer = RE::BSGraphics::Renderer::GetSingleton();
		if (!renderer)
		{
			rlog::error("Cannot find game renderer. Initialization failed.");
			return;
		}

		const auto context   = renderer->data.context;
		const auto swapChain = renderer->data.renderWindows->swapChain;
		const auto forwarder = renderer->data.forwarder;

		rlog::info("Getting DXGI swapchain..."sv);
		auto* swapchain = swapChain;
		if (!swapchain)
		{
			rlog::error("Cannot find game render manager. Initialization failed."sv);
			return;
		}
		rlog::info("Reticulating splines...");

		rlog::info("Getting DXGI swapchain desc..."sv);
		DXGI_SWAP_CHAIN_DESC sd{};
		if (swapchain->GetDesc(std::addressof(sd)) < 0)
		{
			rlog::error("IDXGISwapChain::GetDesc failed."sv);
			return;
		}

		device_  = forwarder;
		context_ = context;

		rlog::info("Initializing ImGui..."sv);
		ImGui::CreateContext();
		if (!ImGui_ImplWin32_Init(sd.OutputWindow))
		{
			rlog::error("ImGui initialization failed (Win32)");
			return;
		}
		if (!ImGui_ImplDX11_Init(device_, context_))
		{
			rlog::error("ImGui initialization failed (DX11)"sv);
			return;
		}

		initialized.store(true);
		rlog::info("Ready to draw the HUD.");

		// Make our blend state for re-use.
		// D3D11_BLEND_DESC desc;
		// ZeroMemory(&desc, sizeof(desc));
		// desc.AlphaToCoverageEnable                 = false;
		// desc.RenderTarget[0].BlendEnable           = true;
		// desc.RenderTarget[0].SrcBlend              = D3D11_BLEND_SRC_ALPHA;
		// desc.RenderTarget[0].DestBlend             = D3D11_BLEND_INV_SRC_ALPHA;
		// desc.RenderTarget[0].BlendOp               = D3D11_BLEND_OP_ADD;
		// desc.RenderTarget[0].SrcBlendAlpha         = D3D11_BLEND_INV_DEST_ALPHA;
		// desc.RenderTarget[0].DestBlendAlpha        = D3D11_BLEND_ONE;
		// desc.RenderTarget[0].BlendOpAlpha          = D3D11_BLEND_OP_ADD;
		// desc.RenderTarget[0].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE_ALL;
		// device_->CreateBlendState(&desc, &gBlendState);

		wnd_proc_hook::func = reinterpret_cast<WNDPROC>(
			SetWindowLongPtrA(sd.OutputWindow, GWLP_WNDPROC, reinterpret_cast<LONG_PTR>(wnd_proc_hook::thunk)));
		if (!wnd_proc_hook::func) { rlog::error("SetWindowLongPtrA failed"sv); }
	}

	void ui_renderer::dxgi_present_hook::thunk(std::uint32_t a_p1)
	{
		func(a_p1);

		if (!d_3d_init_hook::initialized.load()) { return; }

		if (!imFont && !triedFontLoad) { loadFont(); }

		ImGui_ImplDX11_NewFrame();
		ImGui_ImplWin32_NewFrame();
		ImGui::NewFrame();

		// float blendFactor[4] = { 0.0f, 0.0f, 0.0f, 0.0f };
		// UINT sampleMask = 0xffffffff;
		// context_->OMSetBlendState(gBlendState, blendFactor, sampleMask);

		drawHud();

		ImGui::EndFrame();
		ImGui::Render();
		ImGui_ImplDX11_RenderDrawData(ImGui::GetDrawData());
	}

	bool ui_renderer::loadTextureFromFile(const char* filename,
		ID3D11ShaderResourceView** out_srv,
		int32_t& out_width,
		int32_t& out_height)
	{
		auto loadedImg = rasterize_by_path(std::string(filename));
		return d3dTextureFromBuffer(&loadedImg, out_srv, out_width, out_height);
	}

	bool ui_renderer::lazyLoadIcon(std::string name)
	{
		auto key = std::string(get_icon_key(name));
		if (ICON_MAP[key].width > 0) { return true; }

		LoadedImage loadedImg = rasterize_icon(key, MAX_ICON_DIM);
		if (loadedImg.width == 0) { return false; }
		if (d3dTextureFromBuffer(&loadedImg, &ICON_MAP[key].texture, ICON_MAP[key].width, ICON_MAP[key].height))
		{
			rlog::info("Lazy-loaded icon '{}'; width={}; height={}", key, ICON_MAP[key].width, ICON_MAP[key].height);
			return true;
		}
		return false;
	}

	// Helper function to load an image into a DX11 texture with common settings
	bool ui_renderer::d3dTextureFromBuffer(LoadedImage* loadedImg,
		ID3D11ShaderResourceView** out_srv,
		int32_t& out_width,
		int32_t& out_height)
	{
		if (loadedImg->buffer.empty()) { return false; }

		const auto renderer = RE::BSGraphics::Renderer::GetSingleton();
		if (!renderer)
		{
			rlog::error("Cannot find render manager. Initialization failed."sv);
			return false;
		}
		const auto forwarder = renderer->data.forwarder;

		// Create texture
		D3D11_TEXTURE2D_DESC desc;
		ZeroMemory(&desc, sizeof(desc));
		desc.Width            = loadedImg->width;
		desc.Height           = loadedImg->height;
		desc.MipLevels        = 1;
		desc.ArraySize        = 1;
		desc.Format           = DXGI_FORMAT_R8G8B8A8_UNORM;
		desc.SampleDesc.Count = 1;
		desc.Usage            = D3D11_USAGE_DEFAULT;
		desc.BindFlags        = D3D11_BIND_SHADER_RESOURCE;
		desc.CPUAccessFlags   = 0;
		desc.MiscFlags        = 0;

		// copy image_data into the subresource
		auto image_data = (unsigned char*)malloc(loadedImg->buffer.size());
		int counter     = 0;
		for (auto byte : loadedImg->buffer) { image_data[counter++] = static_cast<unsigned char>(byte); }

		ID3D11Texture2D* p_texture = nullptr;
		D3D11_SUBRESOURCE_DATA sub_resource;
		sub_resource.pSysMem          = image_data;
		sub_resource.SysMemPitch      = desc.Width * 4;
		sub_resource.SysMemSlicePitch = 0;
		device_->CreateTexture2D(&desc, &sub_resource, &p_texture);

		// Create texture view
		D3D11_SHADER_RESOURCE_VIEW_DESC srv_desc;
		ZeroMemory(&srv_desc, sizeof srv_desc);
		srv_desc.Format                    = DXGI_FORMAT_R8G8B8A8_UNORM;
		srv_desc.ViewDimension             = D3D11_SRV_DIMENSION_TEXTURE2D;
		srv_desc.Texture2D.MipLevels       = desc.MipLevels;
		srv_desc.Texture2D.MostDetailedMip = 0;
		forwarder->CreateShaderResourceView(p_texture, &srv_desc, out_srv);
		p_texture->Release();

		free(image_data);

		out_width  = loadedImg->width;
		out_height = loadedImg->height;

		return true;
	}

	ui_renderer::ui_renderer() = default;

	void ui_renderer::drawAnimationFrame()
	{
		auto it = animation_list.begin();
		while (it != animation_list.end())
		{
			if (!it->second->is_over())
			{
				auto* anim = it->second.get();

				drawElementInner(animation_frame_map[it->first][anim->current_frame].texture,
					anim->center,
					anim->size,
					anim->angle,
					IM_COL32(anim->r_color, anim->g_color, anim->b_color, anim->alpha));
				anim->animate_action(ImGui::GetIO().DeltaTime);
				++it;
			}
			else { it = animation_list.erase(it); }
		}
	}

	void drawMeterCircleArc(float level, SlotFlattened slotLayout)
	{
		// The flat structure has the same fields to support arc and
		// rectangular meters, so some names might be surprising here.
		const auto meter_center = ImVec2(slotLayout.meter_center.x, slotLayout.meter_center.y);
		const auto meter_size   = ImVec2(slotLayout.meter_size.x, slotLayout.meter_size.y);
		const auto bg_img_str   = std::string(slotLayout.meter_empty_image);
		if (!bg_img_str.empty() && ui_renderer::lazyLoadHudImage(bg_img_str))
		{
			const auto [texture, width, height] = HUD_IMAGES_MAP[bg_img_str];
			drawElement(texture, meter_center, meter_size, 0.0f, slotLayout.meter_empty_color);
		}

		if (meter_size.x != meter_size.y)
		{
			logger::warn("Circular meter is not actually circular. {} != {}", meter_size.x, meter_size.y);
		}
		const auto radius = meter_size.x / 2.0f;
		const auto width  = 10.0f;  // HACK HACK HACK TODO

		ImVec2 start = ImVec2(meter_center.x + radius * cosf(slotLayout.meter_start_angle),
			meter_center.y + radius * sinf(slotLayout.meter_start_angle));
		// level is a percentage IIUC but might not be so this might have to change once I start
		// looking at real values...
		const float startAngle = slotLayout.meter_end_angle;
		const float endAngle   = (slotLayout.meter_end_angle - slotLayout.meter_start_angle) * level / 100.0f;
		ImVec2 end = ImVec2(meter_center.x + radius * cosf(endAngle), meter_center.y + radius * sinf(endAngle));

		const ImU32 fill_color = IM_COL32(slotLayout.meter_fill_color.r,
			slotLayout.meter_fill_color.g,
			slotLayout.meter_fill_color.b,
			slotLayout.meter_fill_color.a * gHudAlpha);

		// ImGui::GetWindowDrawList()->PathLineTo(meter_center);
		ImGui::GetWindowDrawList()->PathClear();
		ImGui::GetWindowDrawList()->PathLineTo(start);
		// IMGUI_API void  PathArcTo(const ImVec2& center, float radius, float a_min, float a_max, int num_segments = 0);
		ImGui::GetWindowDrawList()->PathArcTo(meter_center, radius, startAngle, endAngle, 20);
		ImGui::GetWindowDrawList()->PathLineTo(ImVec2(end.x - width, end.y - width));
		ImGui::GetWindowDrawList()->PathArcTo(meter_center, radius - width, endAngle, startAngle, 20);
		ImGui::GetWindowDrawList()->PathLineToMergeDuplicate(start);
		ImGui::GetWindowDrawList()->PathFillConvex(fill_color);
		ImGui::GetWindowDrawList()->PathClear();
	}

	void drawMeterRectangular(float level, SlotFlattened slotLayout)
	{
		// level is a percent-full level.
		auto missing          = 1.0f - level * 0.01f;
		const auto center     = ImVec2(slotLayout.meter_center.x, slotLayout.meter_center.y);
		const auto bgSize     = ImVec2(slotLayout.meter_size.x, slotLayout.meter_size.y);
		const auto bg_img_str = std::string(slotLayout.meter_empty_image);
		const auto fg_img_str = std::string(slotLayout.meter_fill_image);

		auto angle       = slotLayout.meter_start_angle;
		bool haveBgImage = bg_img_str.empty() ? false : ui_renderer::lazyLoadHudImage(bg_img_str);
		bool haveFgImage = fg_img_str.empty() ? false : ui_renderer::lazyLoadHudImage(fg_img_str);

		if (haveBgImage && haveFgImage)
		{
			const auto [bgtex, width, height]   = HUD_IMAGES_MAP[bg_img_str];
			const auto [fgtex, fwidth, fheight] = HUD_IMAGES_MAP[fg_img_str];
			const auto fillLen                  = slotLayout.meter_fill_size.x * level * 0.01f;
			const auto fillSize                 = ImVec2(fillLen, slotLayout.meter_fill_size.y);
			const auto offset                   = slotLayout.meter_center.x - fillLen * 0.5f;
			const auto fillCenter               = ImVec2(slotLayout.meter_center.x - offset, slotLayout.meter_center.y);

			const std::array<ImVec2, 4> bgRotated = rotateRect(center, bgSize, angle);
			const std::array<ImVec2, 4> centerRot = rotateRect(center, fillSize, angle);
			// now slide that fg rect down to nestle in the bottom left corner of the bg rect
			const auto xdiff = bgRotated[0].x - centerRot[0].x - std::fabs(bgSize.x - fillSize.x) * 0.5f;
			const auto ydiff = bgRotated[0].y - centerRot[0].y - std::fabs(bgSize.y - fillSize.y) * 0.5f;
			std::array<ImVec2, 4> fgRotated = {
				ImVec2(centerRot[0].x - xdiff, centerRot[0].y + ydiff),
				ImVec2(centerRot[1].x - xdiff, centerRot[1].y + ydiff),
				ImVec2(centerRot[2].x - xdiff, centerRot[2].y + ydiff),
				ImVec2(centerRot[3].x - xdiff, centerRot[3].y + ydiff),
			};

			drawTextureQuad(bgtex, bgRotated, slotLayout.meter_empty_color);
			drawTextureQuad(fgtex, fgRotated, slotLayout.meter_fill_color);
			// drawElement(bgtex, center, bgSize, angle, slotLayout.meter_empty_color);
			// drawElement(fgtex, center, fillSize, angle, slotLayout.meter_fill_color);
		}
		else if (haveBgImage && !haveFgImage)
		{
			// Here we draw the bg image twice: once at full size for the empty background,
			// and a second time with the fill color, clipped to indicate charge level.
			auto adjust_x = slotLayout.meter_size.x * missing;
			auto adjust_y = 0.0f;

			// clip_min is left, top
			const auto clip_min = ImVec2(center.x - bgSize.x / 2.0f, center.y - bgSize.y / 2.0f + adjust_y);
			// clip_max is right, bottom
			const auto clip_max = ImVec2(center.x + bgSize.x / 2.0f - adjust_x, center.y + bgSize.y / 2.0f);
			// rotate the clip rect wheeeeee high school trig
			const auto rotMin = ImVec2(clip_min.x * std::cosf(angle) - std::sinf(angle) * clip_min.y,
				std::sinf(angle) * clip_min.x + std::cosf(angle) * clip_min.y);
			const auto rotMax = ImVec2(clip_max.x * std::cosf(angle) - std::sinf(angle) * clip_max.y,
				std::sinf(angle) * clip_max.x + std::cosf(angle) * clip_max.y);

			const auto [texture, width, height] = HUD_IMAGES_MAP[bg_img_str];
			drawElement(texture, center, bgSize, angle, slotLayout.meter_empty_color);
			// IMGUI_API void          PushClipRect(const ImVec2& clip_rect_min, const ImVec2& clip_rect_max, bool intersect_with_current_clip_rect);
			ImGui::GetWindowDrawList()->PushClipRect(rotMin, rotMax, true);
			drawElement(texture, center, bgSize, angle, slotLayout.meter_fill_color);
			ImGui::GetWindowDrawList()->PopClipRect();
		}
		else
		{
			// TODO if no svg, do rectangular flood fills? idk
		}
	}

	void drawText(const std::string text,
		const ImVec2 center,
		const float fontSize,
		const Color color,
		const Align align,
		const float wrapWidth)
	{
		if (!text.length() || color.a == 0) { return; }

		const ImU32 text_color   = IM_COL32(color.r, color.g, color.b, color.a * gHudAlpha);
		const ImVec2 text_bounds = ImGui::CalcTextSize(text.c_str());
		auto* font               = imFont;
		if (!font) { font = ImGui::GetDefaultFont(); }

		// Listen up, future maintainer aka ceej of the future!
		// Text alignment is, for cognitive ease reasons, also a statement about
		// where the stated anchor point is.
		//
		// Center alignment: the offset refers to the center of the text box. (easy case!)
		// Left alignment: the offset refers to the center of the left edge.
		// Right alignment: the offset refers to the center of the right edge.
		//
		// Since imgui takes a *LEFT* edge point, we have to offset the other two cases
		// by an amount that depends on the size of the text box.
		// Center alignment: offset to the left by half.
		// Right alignment: offset to the left by the entire length of the box.
		float adjustment = 0;
		if (align == Align::Center) { adjustment = -0.5f * text_bounds.x; }
		else if (align == Align::Right) { adjustment = -1.0f * text_bounds.x; }

		ImVec2 aligned_loc = ImVec2(center.x + adjustment, center.y);

		ImGui::GetWindowDrawList()->AddText(
			font, fontSize, aligned_loc, text_color, text.c_str(), nullptr, wrapWidth, nullptr);
	}

	void ui_renderer::initializeAnimation(const animation_type animation_type,
		const float a_screen_x,
		const float a_screen_y,
		const float a_offset_x,
		const float a_offset_y,
		const float width,
		const float height,
		const uint32_t a_modify,
		const uint32_t a_alpha,
		float a_duration)
	{
		if (a_alpha == 0) { return; }

		// rlog::trace("starting inited animation");
		constexpr auto angle = 0.0f;

		const auto size = static_cast<uint32_t>(animation_frame_map[animation_type].size());
		// const auto width  = static_cast<uint32_t>(animation_frame_map[animation_type][0].width);
		// const auto height = static_cast<uint32_t>(animation_frame_map[animation_type][0].height);

		std::unique_ptr<Animation> anim =
			std::make_unique<FadeFramedOutAnimation>(ImVec2(a_screen_x + a_offset_x, a_screen_y + a_offset_y),
				ImVec2(width, height),
				angle,
				a_alpha,
				a_modify,
				a_modify,
				a_modify,
				a_duration,
				size);
		animation_list.emplace_back(static_cast<ui::animation_type>(animation_type), std::move(anim));
	}

	void drawElement(ID3D11ShaderResourceView* texture,
		const ImVec2 center,
		const ImVec2 size,
		const float angle,
		const Color color)
	{
		const ImU32 im_color = IM_COL32(color.r, color.g, color.b, color.a * gHudAlpha);
		drawElementInner(texture, center, size, angle, im_color);
	}

	std::array<ImVec2, 4> rotateRect(const ImVec2 center, const ImVec2 size, const float angle)
	{
		std::array<ImVec2, 4> rotated;
		const float cos_a = cosf(angle);
		const float sin_a = sinf(angle);
		rotated           = { center + ImRotate(ImVec2(-size.x * 0.5f, -size.y * 0.5f), cos_a, sin_a),
					  center + ImRotate(ImVec2(+size.x * 0.5f, -size.y * 0.5f), cos_a, sin_a),
					  center + ImRotate(ImVec2(+size.x * 0.5f, +size.y * 0.5f), cos_a, sin_a),
					  center + ImRotate(ImVec2(-size.x * 0.5f, +size.y * 0.5f), cos_a, sin_a)

		};
		return rotated;
	}

	void drawElementInner(ID3D11ShaderResourceView* texture,
		const ImVec2 center,
		const ImVec2 size,
		const float angle,
		const ImU32 im_color)
	{
		// const float cos_a   = cosf(angle);
		// const float sin_a   = sinf(angle);
		// const ImVec2 pos[4] = { center + ImRotate(ImVec2(-size.x * 0.5f, -size.y * 0.5f), cos_a, sin_a),
		// 	center + ImRotate(ImVec2(+size.x * 0.5f, -size.y * 0.5f), cos_a, sin_a),
		// 	center + ImRotate(ImVec2(+size.x * 0.5f, +size.y * 0.5f), cos_a, sin_a),
		// 	center + ImRotate(ImVec2(-size.x * 0.5f, +size.y * 0.5f), cos_a, sin_a)

		std::array<ImVec2, 4> pos = rotateRect(center, size, angle);
		ImGui::GetWindowDrawList()->AddImageQuad(
			texture, pos[0], pos[1], pos[2], pos[3], FLAT_UVS[0], FLAT_UVS[1], FLAT_UVS[2], FLAT_UVS[3], im_color);
	}

	void drawTextureQuad(ID3D11ShaderResourceView* texture, const std::array<ImVec2, 4> bounds, const Color color)
	{
		const ImU32 im_color = IM_COL32(color.r, color.g, color.b, color.a * gHudAlpha);
		ImGui::GetWindowDrawList()->AddImageQuad(texture,
			bounds[0],
			bounds[1],
			bounds[2],
			bounds[3],
			FLAT_UVS[0],
			FLAT_UVS[1],
			FLAT_UVS[2],
			FLAT_UVS[3],
			im_color);
	}

	void drawAllSlots()
	{
		auto topLayout          = hud_layout();
		auto anchor             = topLayout.anchor;
		auto hudsize            = topLayout.bg_size;
		bool rangedEquipped     = player::hasRangedEquipped();
		const auto settings     = user_settings();
		const auto screenWidth  = resolutionWidth();
		const auto screenHeight = resolutionHeight();
		bool colorizeIcons      = settings->colorize_icons();

		// If the layout is larger than the HUD, restrict it to one quarter screen size.
		hudsize.x = std::min(screenWidth / 4.0f, hudsize.x);
		hudsize.y = std::min(screenHeight / 4.0f, hudsize.y);

		// If the layout is trying to draw the HUD offscreen, clamp it to an edge.
		anchor.x = std::clamp(anchor.x, hudsize.x / 2.0f, screenWidth - hudsize.x / 2.0f);
		anchor.y = std::clamp(anchor.y, hudsize.y / 2.0f, screenHeight - hudsize.y / 2.0f);

		// Draw the HUD background if requested.
		const auto bgimg = std::string(topLayout.bg_image);
		if (topLayout.bg_color.a > 0 && ui_renderer::lazyLoadHudImage(bgimg))
		{
			constexpr auto angle                = 0.f;
			const auto center                   = ImVec2(anchor.x, anchor.y);
			const auto [texture, width, height] = HUD_IMAGES_MAP[bgimg];
			const auto size                     = ImVec2(hudsize.x, hudsize.y);
			drawElement(texture, center, size, angle, topLayout.bg_color);
		}

		for (auto slotLayout : topLayout.slots)
		{
			if ((slotLayout.element == HudElement::Left) && topLayout.hide_left_when_irrelevant && rangedEquipped)
			{
				continue;
			}
			if ((slotLayout.element == HudElement::Ammo) && topLayout.hide_ammo_when_irrelevant && !rangedEquipped)
			{
				continue;
			}

			rust::Box<HudItem> entry = entry_to_show_in_slot(slotLayout.element);
			if ((slotLayout.element == HudElement::EquipSet) && entry->name().empty())
			{
				// Do nothing for empty equipsets. TODO draw as empty slot
				continue;
			}

			auto entry_name = std::string("");
			// We use the data cached in the entry if at all possible
			if (entry->name_is_utf8()) { entry_name = std::string(entry->name()); }
			else
			{
				// use the bytes from the cstring, which are identical to the data the form gave us
				// note that imgui cannot draw non-utf8-valid characters, so we'll get the ?? subs.
				// I am *guessing* that the Flash menus are old enough that they handle UCS-16 BE
				// data, which is why people do it. OMFG this explains the translation files too.
				auto bytes = entry->name_bytes();
				entry_name = helpers::vec_to_stdstring(bytes);
			}

			const auto hotkey      = settings->hotkey_for(slotLayout.element);
			const auto slot_center = ImVec2(slotLayout.center.x, slotLayout.center.y);

			const auto slotbg = std::string(slotLayout.bg_image);
			if (slotLayout.bg_color.a > 0 && ui_renderer::lazyLoadHudImage(slotbg))
			{
				const auto [texture, width, height] = HUD_IMAGES_MAP[slotbg];
				const auto size                     = ImVec2(slotLayout.bg_size.x, slotLayout.bg_size.y);
				drawElement(texture, slot_center, size, 0.f, slotLayout.bg_color);
			}

			// now draw the icon over the background...
			if (slotLayout.icon_color.a > 0)
			{
				const auto iconColor = colorizeIcons ? entry->color() : slotLayout.icon_color;
				auto iconkey         = std::string(entry->icon_key());
				if (ui_renderer::lazyLoadIcon(iconkey))
				{
					const auto [texture, width, height] = ICON_MAP[iconkey];
					const auto scale =
						width > height ? (slotLayout.icon_size.x / width) : (slotLayout.icon_size.y / height);
					const auto size     = ImVec2(width * scale, height * scale);
					const auto icon_pos = ImVec2(slotLayout.icon_center.x, slotLayout.icon_center.y);

					drawElement(texture, icon_pos, size, 0.f, iconColor);
				}
				else { rlog::debug("lazy load for icon key {} failed; not drawing icon.", iconkey); }
			}

			// Loop through the text elements of this slot.
			for (auto label : slotLayout.text)
			{
				if (label.color.a == 0) { continue; }
				const auto textPos = ImVec2(label.anchor.x, label.anchor.y);
				auto entrytxt      = std::string(entry->fmtstr(label.contents));
				// Let's try a wrap width here. This is going to be wrong, but we'll experiment.
				if (!entrytxt.empty())
				{
					drawText(entrytxt, textPos, label.font_size, label.color, label.alignment, label.wrap_width);
				}
			}

			// Draw the hotkey reminder if asked.
			if (slotLayout.hotkey_color.a > 0)
			{
				const auto hk_im_center = ImVec2(slotLayout.hotkey_center.x, slotLayout.hotkey_center.y);

				const auto hotkeybg = std::string(slotLayout.hotkey_bg_image);
				if (slotLayout.hotkey_bg_color.a > 0 && ui_renderer::lazyLoadHudImage(hotkeybg))
				{
					const auto [texture, width, height] = HUD_IMAGES_MAP[hotkeybg];
					const auto size                     = ImVec2(slotLayout.hotkey_size.x, slotLayout.hotkey_size.y);
					drawElement(texture, hk_im_center, size, 0.f, slotLayout.hotkey_bg_color);
				}

				const auto [texture, width, height] = ui_renderer::iconForHotkey(hotkey);
				const auto size                     = ImVec2(static_cast<float>(slotLayout.hotkey_size.x - 2.0f),
                    static_cast<float>(slotLayout.hotkey_size.y - 2.0f));
				drawElement(texture, hk_im_center, size, 0.f, slotLayout.hotkey_color);
			}

			// Charge/fuel meter.
			if (slotLayout.meter_kind != MeterKind::None && entry->has_charge())
			{
				auto level = entry->charge_level();
				if (slotLayout.meter_kind == MeterKind::CircleArc) { drawMeterCircleArc(level, slotLayout); }
				else if (slotLayout.meter_kind == MeterKind::Rectangular) { drawMeterRectangular(level, slotLayout); }
			}

			// Finally, the poisoned indicator.
			if (slotLayout.poison_color.a > 0 && entry->is_poisoned())
			{
				const auto poison_img = std::string(slotLayout.poison_image);
				if (ui_renderer::lazyLoadHudImage(poison_img))
				{
					const auto poison_center = ImVec2(slotLayout.poison_center.x, slotLayout.poison_center.y);
					const auto [texture, width, height] = HUD_IMAGES_MAP[poison_img];
					const auto size                     = ImVec2(slotLayout.poison_size.x, slotLayout.poison_size.y);
					drawElement(texture, poison_center, size, 0.f, slotLayout.poison_color);
				}
			}
		}

		// drawAnimationFrame();
	}

	void drawHud()
	{
		const auto timeDelta = ImGui::GetIO().DeltaTime;
		advanceTimers(timeDelta);

		if (!helpers::hudAllowedOnScreen()) return;
		makeFadeDecision();
		advanceTransition(timeDelta);
		if (gHudAlpha == 0.0f) { return; }

		static constexpr ImGuiWindowFlags window_flags =
			ImGuiWindowFlags_NoBackground | ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_NoInputs;

		const float screen_size_x = ImGui::GetIO().DisplaySize.x, screen_size_y = ImGui::GetIO().DisplaySize.y;

		ImGui::SetNextWindowSize(ImVec2(screen_size_x, screen_size_y));
		ImGui::SetNextWindowPos(ImVec2(0.f, 0.f));
		ImGui::GetStyle().Alpha = gHudAlpha;

		ImGui::Begin(HUD_NAME, nullptr, window_flags);

		drawAllSlots();

		ImGui::End();
	}

	template <typename T>
	void ui_renderer::loadImagesForMap(std::map<std::string, T>& imagesMap,
		std::map<uint32_t, TextureData>& textureCache,
		std::string& imgDirectory)
	{
		const auto res_width  = resolutionScaleWidth();
		const auto res_height = resolutionScaleHeight();

		for (const auto& entry : std::filesystem::directory_iterator(imgDirectory))
		{
			if (imagesMap.contains(entry.path().filename().string()))
			{
				if (entry.path().filename().extension() != ".svg")
				{
					rlog::warn("file {}, does not match supported extension '.svg'"sv,
						entry.path().filename().string().c_str());
					continue;
				}
				const auto index = static_cast<int32_t>(imagesMap[entry.path().filename().string()]);
				if (loadTextureFromFile(entry.path().string().c_str(),
						&textureCache[index].texture,
						textureCache[index].width,
						textureCache[index].height))
				{
					/*
					rlog::trace("loading texture {}, type: {}, width: {}, height: {}"sv,
						entry.path().filename().string().c_str(),
						entry.path().filename().extension().string().c_str(),
						textureCache[index].width,
						textureCache[index].height);
					*/
				}
				else { rlog::error("failed to load texture {}"sv, entry.path().filename().string().c_str()); }

				textureCache[index].width  = static_cast<int32_t>(textureCache[index].width * res_width);
				textureCache[index].height = static_cast<int32_t>(textureCache[index].height * res_height);
			}
		}
	}

	void ui_renderer::loadAnimationFrames(std::string& file_path, std::vector<TextureData>& frame_list)
	{
		for (const auto& entry : std::filesystem::directory_iterator(file_path))
		{
			ID3D11ShaderResourceView* texture = nullptr;
			int32_t width                     = 0;
			int32_t height                    = 0;
			if (entry.path().filename().extension() != ".svg")
			{
				rlog::warn(
					"file {}, does not match supported extension '.svg'"sv, entry.path().filename().string().c_str());
				continue;
			}

			loadTextureFromFile(entry.path().string().c_str(), &texture, width, height);

			// rlog::trace("loading animation frame: {}"sv, entry.path().string().c_str());
			TextureData img;
			img.texture = texture;
			// img.width   = static_cast<int32_t>(width * resolutionScaleWidth());
			// img.height  = static_cast<int32_t>(height * resolutionScaleHeight());
			frame_list.push_back(img);
		}
	}

	TextureData ui_renderer::iconForHotkey(const uint32_t a_key)
	{
		const auto settings = user_settings();
		auto return_image   = default_key_struct[static_cast<int32_t>(default_keys::key)];
		// todo rework this logic at some point, no rush
		if (a_key >= keycodes::kGamepadOffset)
		{
			if (settings->controller_kind() == static_cast<uint32_t>(controller_set::playstation))
			{
				return_image = PS5_BUTTON_MAP[a_key];
			}
			else { return_image = XBOX_BUTTON_MAP[a_key]; }
		}
		else
		{
			if (key_struct.contains(a_key)) { return_image = key_struct[a_key]; }
		}
		return return_image;
	}

	bool ui_renderer::lazyLoadHudImage(std::string key)
	{
		if (HUD_IMAGES_MAP[key].width > 0) { return true; }
		std::string path      = R"(Data\SKSE\Plugins\resources\backgrounds\)" + key;
		LoadedImage loadedImg = rasterize_by_path(path);
		if (loadedImg.width == 0) { return false; }
		if (d3dTextureFromBuffer(
				&loadedImg, &HUD_IMAGES_MAP[key].texture, HUD_IMAGES_MAP[key].width, HUD_IMAGES_MAP[key].height))
		{
			rlog::info("Lazy-loaded hud bg image '{}'; width={}; height={}",
				key,
				HUD_IMAGES_MAP[key].width,
				HUD_IMAGES_MAP[key].height);
			return true;
		}
		rlog::warn("Failed to load requested hud image '{}'; double-check the svg name in the layout file!", key);
		return false;
	}

	void ui_renderer::loadFont()
	{
		auto hud         = hud_layout();
		auto fontfile    = std::string(hud.font);
		std::string path = R"(Data\SKSE\Plugins\resources\fonts\)" + fontfile;
		auto file_path   = std::filesystem::path(path);

		rlog::trace(
			"about to try to load font; size={}; globalScale={}; path={}"sv, hud.font_size, hud.global_scale, path);
		triedFontLoad = true;
		if (std::filesystem::is_regular_file(file_path) &&
			((file_path.extension() == ".ttf") || (file_path.extension() == ".otf")))
		{
			ImGuiIO& io = ImGui::GetIO();
			ImVector<ImWchar> ranges;
			ImFontGlyphRangesBuilder builder;

			builder.AddRanges(io.Fonts->GetGlyphRangesDefault());
			if (hud.chinese_full_glyphs) { builder.AddRanges(io.Fonts->GetGlyphRangesChineseFull()); }
			if (hud.simplified_chinese_glyphs) { builder.AddRanges(io.Fonts->GetGlyphRangesChineseSimplifiedCommon()); }
			if (hud.cyrillic_glyphs) { builder.AddRanges(io.Fonts->GetGlyphRangesCyrillic()); }
			if (hud.japanese_glyphs) { builder.AddRanges(io.Fonts->GetGlyphRangesJapanese()); }
			if (hud.korean_glyphs) { builder.AddRanges(io.Fonts->GetGlyphRangesKorean()); }
			if (hud.thai_glyphs) { builder.AddRanges(io.Fonts->GetGlyphRangesThai()); }
			if (hud.vietnamese_glyphs) { builder.AddRanges(io.Fonts->GetGlyphRangesVietnamese()); }

			builder.BuildRanges(&ranges);
			auto scaledSize = hud.font_size;

			imFont = io.Fonts->AddFontFromFileTTF(file_path.string().c_str(), scaledSize, nullptr, ranges.Data);
			if (io.Fonts->Build())
			{
				ImGui_ImplDX11_CreateDeviceObjects();
				rlog::info("font loaded; path={}"sv, path);
				return;
			}
		}
	}

	void ui_renderer::preloadImages()
	{
		loadImagesForMap(key_icon_name_map, key_struct, key_directory);
		loadImagesForMap(default_key_icon_name_map, default_key_struct, key_directory);
		loadImagesForMap(gamepad_ps_icon_name_map, PS5_BUTTON_MAP, key_directory);
		loadImagesForMap(gamepad_xbox_icon_name_map, XBOX_BUTTON_MAP, key_directory);

		loadAnimationFrames(highlight_animation_directory, animation_frame_map[animation_type::highlight]);
		rlog::trace("frame length is {}"sv, animation_frame_map[animation_type::highlight].size());
	}

	// These values scale the UI from the resolution the mod author used to the resolution
	// of the player's screen. The effect is to make things not over-large for smaller resolutions.
	// TODO this should be restored BUT as a lookup from the layout file. The designer can
	// state their intent.
	// float ui_renderer::resolutionScaleWidth() { return ImGui::GetIO().DisplaySize.x / 1920.f; }
	// float ui_renderer::resolutionScaleHeight() { return ImGui::GetIO().DisplaySize.y / 1080.f; }

	float resolutionScaleWidth() { return 1.0f; }
	float resolutionScaleHeight() { return 1.0f; }

	float resolutionWidth() { return ImGui::GetIO().DisplaySize.x; }
	float resolutionHeight() { return ImGui::GetIO().DisplaySize.y; }

	void showBriefly()
	{
		if (gDoingBriefPeek || gHudAlpha >= gMaxAlpha || (doFadeIn == true && gHudAlpha > 0.0f)) { return; }

		gDoingBriefPeek = true;
		startAlphaTransition(true, gMaxAlpha);
	}

	void setMaxAlpha(float max)
	{
		gMaxAlpha = std::clamp(std::abs(max), 0.2f, 1.0f);
		if (gHudAlpha > gMaxAlpha) { gHudAlpha = gMaxAlpha; }
	}

	void startAlphaTransition(const bool becomeVisible, const float goal)
	{
		gGoalAlpha = std::clamp(goal, 0.0f, gMaxAlpha);
		if (becomeVisible && gHudAlpha >= gMaxAlpha) { return; }
		if (!becomeVisible && gHudAlpha == 0.0f) { return; }
		rlog::debug("startAlphaTransition() called with in={} and goal={}; gHudAlpha={};"sv,
			becomeVisible,
			gGoalAlpha,
			gHudAlpha);

		doFadeIn = becomeVisible;

		// The game will report that the player has sheathed weapons when
		// the player has merely equipped something new. So we give it some
		// time to decide that the weapons are truly gone. This number is the
		// how long we'll wait before actually fading.
		if (!doFadeIn) { delayBeforeFadeout = FADEOUT_HYSTERESIS; }

		auto settings   = user_settings();
		float fade_time = static_cast<float>(settings->fade_time()) / 1000.0f;
		if (gDoingBriefPeek)
		{
			fade_time = fade_time / 2.0f;  // fastest fade-in in the west
		}
		gFullFadeDuration = doFadeIn ? (fade_time / 2.0f) : fade_time;  // fade in is faster than fade out

		// We must allow for the transition starting while the alpha is not pinned.
		// Scale the transition time for how much of the shift remains.
		auto alphaGap     = std::fabs(gGoalAlpha - gHudAlpha);
		gFadeDurRemaining = alphaGap * gFullFadeDuration;
		if (gFadeDurRemaining < 0.005f)
		{
			// Not enough time to bother fading. Just snap to the goal.
			gHudAlpha         = gGoalAlpha;
			gFadeDurRemaining = 0.0f;
			return;
		}

		gIsFading = true;
	}

	void makeFadeDecision()
	{
		auto settings = user_settings();
		bool autofade = settings->autofade();

		// We do the peek even when autofade is false, so we need to fade out automatically in that one case.
		if (!autofade)
		{
			if (gDoingBriefPeek && gHudAlpha >= gMaxAlpha)
			{
				gDoingBriefPeek = false;
				startAlphaTransition(false, 0.0f);
			}
			return;
		}

		// Now the autofade decision.
		if (helpers::hudShouldAutoFadeOut())
		{
			if (gDoingBriefPeek)
			{
				if (gHudAlpha < gMaxAlpha) { return; }
				gDoingBriefPeek = false;
			}
			// The auto-fade case here.
			if ((gHudAlpha > 0.0f && !gIsFading) || (gIsFading && doFadeIn)) { startAlphaTransition(false, 0.0f); }
		}
		else if (helpers::hudShouldAutoFadeIn())
		{
			if ((gHudAlpha < gMaxAlpha && !gIsFading) || (gIsFading && !doFadeIn))
			{
				startAlphaTransition(true, gMaxAlpha);
			}
		}
	}

	float easeInCubic(float progress)
	{
		if (progress >= 1.0f) return 1.0f;
		if (progress <= 0.0f) return 0.0f;
		return static_cast<float>(pow(progress, 3));
	}

	float easeOutCubic(float progress)
	{
		if (progress >= 1.0f) return 1.0f;
		if (progress <= 0.0f) return 0.0f;
		return static_cast<float>(1.0f - pow(1 - progress, 3));
	}

	void advanceTransition(float timeDelta)
	{
		// This fading code is triggered by the toggle hud shortcut even if autofade
		// is off. This is maybe the only place where bug #44 might be caused.
		if (doFadeIn && gIsFading)
		{
			if (gHudAlpha >= gMaxAlpha)
			{
				gHudAlpha         = gMaxAlpha;
				gFadeDurRemaining = 0.0f;
				gIsFading         = false;
				return;
			}
			if (gFadeDurRemaining > 0.0f) { gFadeDurRemaining -= timeDelta; }
			gHudAlpha = easeInCubic(1.0f - (gFadeDurRemaining / gFullFadeDuration));
		}
		else if (!doFadeIn && gIsFading)
		{
			if (delayBeforeFadeout > 0.0f) { delayBeforeFadeout -= timeDelta; }
			else
			{
				if (gHudAlpha <= 0.0f)
				{
					gHudAlpha         = 0.0f;
					gFadeDurRemaining = 0.0f;
					gIsFading         = false;
				}
				delayBeforeFadeout = 0.0f;
				if (gFadeDurRemaining > 0.0f) { gFadeDurRemaining -= timeDelta; }
				gHudAlpha = gMaxAlpha - easeInCubic(1.0f - (gFadeDurRemaining / gFullFadeDuration));
			}
		}
	}

	// We implement timers using UI ticks. We don't need them to be
	// particularly accurate, just good-feeling to humans. Because we only
	// manage timers here, this is the right decision point for going into
	// and out of slow motion.
	void advanceTimers(float delta)
	{
		std::vector<uint8_t> to_remove;
		std::map<uint8_t, float>::iterator iter;
		for (iter = cycle_timers.begin(); iter != cycle_timers.end(); ++iter)
		{
			auto which     = iter->first;
			auto remaining = iter->second;

			remaining -= delta;
			// rlog::trace("timer decremented; timer={}; delta={}; remaining={};"sv, which, delta, remaining);
			if (remaining < 0.0f)
			{
				to_remove.push_back(which);
				auto action = static_cast<Action>(which);
				timer_expired(action);
			}
			else { cycle_timers[which] = remaining; }
		}

		for (const auto& xs : to_remove) { cycle_timers.erase(xs); }
		if (cycle_timers.size() == 0) { helpers::exitSlowMotion(); }
	}

	void startTimer(Action which, uint32_t duration)
	{
		// We replace any existing timer for this slot.
		// All incoming durations are in milliseconds, but our time deltas
		// are floats where whole numbers are seconds. So we divide.
		const auto settings = user_settings();
		cycle_timers.insert_or_assign(static_cast<uint8_t>(which), static_cast<float>(duration) / 1000.0f);
		rlog::debug("Started equip delay timer; which={}; duration={} ms;"sv, static_cast<uint8_t>(which), duration);
		// TODO do not start slomo for long-presses???
		if (settings->cycling_slows_time() && RE::PlayerCharacter::GetSingleton()->IsInCombat())
		{
			helpers::enterSlowMotion();
		}
	}

	// remove timer from the map if it exists
	void stopTimer(Action which)
	{
		cycle_timers.erase(static_cast<uint8_t>(which));
		if (cycle_timers.size() == 0) { helpers::exitSlowMotion(); }
	}
}
