#include "ui_renderer.h"
#include "animation_handler.h"
#include "constant.h"
#include "gear.h"
#include "helpers.h"
#include "image_path.h"
#include "key_path.h"
#include "keycodes.h"

#include "lib.rs.h"

namespace ui
{
	static std::map<animation_type, std::vector<TextureData>> animation_frame_map = {};
	static std::vector<std::pair<animation_type, std::unique_ptr<Animation>>> animation_list;

	static std::map<uint8_t, float> cycle_timers = {};

	static std::map<uint32_t, TextureData> image_struct;
	static std::map<uint32_t, TextureData> key_struct;
	static std::map<uint32_t, TextureData> default_key_struct;
	static std::map<uint32_t, TextureData> PS5_BUTTON_MAP;
	static std::map<uint32_t, TextureData> XBOX_BUTTON_MAP;
	static std::map<std::string, TextureData> ICON_MAP;

	static const float FADEOUT_HYSTERESIS = 0.5f;  // seconds
	static const uint32_t MAX_ICON_DIM    = 300;   // rasterized at 96 dpi

	auto gHudAlpha          = 0.0f;
	auto gGoalAlpha         = 1.0f;
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

		logger::info("D3DInit hooked so we can give imgui something to render to."sv);
		const auto renderer = RE::BSGraphics::Renderer::GetSingleton();
		if (!renderer)
		{
			logger::error("Cannot find game renderer. Initialization failed.");
			return;
		}

		const auto context   = renderer->data.context;
		const auto swapChain = renderer->data.renderWindows->swapChain;
		const auto forwarder = renderer->data.forwarder;

		logger::info("Getting DXGI swapchain..."sv);
		auto* swapchain = swapChain;
		if (!swapchain)
		{
			logger::error("Cannot find game render manager. Initialization failed."sv);
			return;
		}
		logger::info("Reticulating splines...");

		logger::info("Getting DXGI swapchain desc..."sv);
		DXGI_SWAP_CHAIN_DESC sd{};
		if (swapchain->GetDesc(std::addressof(sd)) < 0)
		{
			logger::error("IDXGISwapChain::GetDesc failed."sv);
			return;
		}

		device_  = forwarder;
		context_ = context;

		logger::info("Initializing ImGui..."sv);
		ImGui::CreateContext();
		if (!ImGui_ImplWin32_Init(sd.OutputWindow))
		{
			logger::error("ImGui initialization failed (Win32)");
			return;
		}
		if (!ImGui_ImplDX11_Init(device_, context_))
		{
			logger::error("ImGui initialization failed (DX11)"sv);
			return;
		}

		initialized.store(true);
		logger::info("Ready to draw the HUD.");

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
		if (!wnd_proc_hook::func) { logger::error("SetWindowLongPtrA failed"sv); }
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
			logger::info("Lazy-loaded icon '{}'; width={}; height={}", key, ICON_MAP[key].width, ICON_MAP[key].height);
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
			logger::error("Cannot find render manager. Initialization failed."sv);
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

	void ui_renderer::draw_animations_frame()
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

	void ui_renderer::drawText(const std::string text,
		const ImVec2 center,
		const float font_size,
		const Color color,
		const Align align)
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
			font, font_size, aligned_loc, text_color, text.c_str(), nullptr, 0.0f, nullptr);
	}

	void ui_renderer::init_animation(const animation_type animation_type,
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

		// logger::trace("starting inited animation");
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
		// logger::trace("done initializing animation. return.");
	}

	void ui_renderer::drawElement(ID3D11ShaderResourceView* texture,
		const ImVec2 center,
		const ImVec2 size,
		const float angle,
		const Color color)
	{
		const ImU32 im_color = IM_COL32(color.r, color.g, color.b, color.a * gHudAlpha);
		drawElementInner(texture, center, size, angle, im_color);
	}

	void ui_renderer::drawElementInner(ID3D11ShaderResourceView* texture,
		const ImVec2 center,
		const ImVec2 size,
		const float angle,
		const ImU32 im_color)
	{
		const float cos_a   = cosf(angle);
		const float sin_a   = sinf(angle);
		const ImVec2 pos[4] = { center + ImRotate(ImVec2(-size.x * 0.5f, -size.y * 0.5f), cos_a, sin_a),
			center + ImRotate(ImVec2(+size.x * 0.5f, -size.y * 0.5f), cos_a, sin_a),
			center + ImRotate(ImVec2(+size.x * 0.5f, +size.y * 0.5f), cos_a, sin_a),
			center + ImRotate(ImVec2(-size.x * 0.5f, +size.y * 0.5f), cos_a, sin_a)

		};
		constexpr ImVec2 uvs[4] = { ImVec2(0.0f, 0.0f), ImVec2(1.0f, 0.0f), ImVec2(1.0f, 1.0f), ImVec2(0.0f, 1.0f) };

		ImGui::GetWindowDrawList()->AddImageQuad(
			texture, pos[0], pos[1], pos[2], pos[3], uvs[0], uvs[1], uvs[2], uvs[3], im_color);
	}

	void ui_renderer::drawAllSlots()
	{
		auto topLayout          = hud_layout();
		auto anchor             = topLayout.anchor_point();
		auto hudsize            = topLayout.size;
		bool rangedEquipped     = player::hasRangedEquipped();
		const auto settings     = user_settings();
		const auto screenWidth  = resolutionWidth();
		const auto screenHeight = resolutionHeight();
		bool colorizeIcons      = settings->colorize_icons();

		auto globalScale = topLayout.global_scale;
		// serde's default for missing f32 fields is 0
		if (globalScale == 0.0f) { globalScale = 1.0f; }

		// If the layout is larger than the HUD, restrict it to one quarter screen size.
		hudsize.x = std::min(screenWidth / 4.0f, globalScale * hudsize.x);
		hudsize.y = std::min(screenHeight / 4.0f, globalScale * hudsize.y);

		// If the layout is trying to draw the HUD offscreen, clamp it to an edge.
		anchor.x = std::clamp(anchor.x, hudsize.x / 2.0f, screenWidth - hudsize.x / 2.0f);
		anchor.y = std::clamp(anchor.y, hudsize.y / 2.0f, screenHeight - hudsize.y / 2.0f);

		// Draw the HUD background if requested.
		if (topLayout.bg_color.a > 0)
		{
			constexpr auto angle                = 0.f;
			const auto center                   = ImVec2(anchor.x, anchor.y);
			const auto [texture, width, height] = image_struct[static_cast<int32_t>(image_type::hud)];
			const auto size                     = ImVec2(hudsize.x, hudsize.y);
			drawElement(texture, center, size, angle, topLayout.bg_color);
		}

		for (auto slotLayout : topLayout.layouts)
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

			const auto hotkey = settings->hotkey_for(slotLayout.element);
			const auto slot_center =
				ImVec2(anchor.x + slotLayout.offset.x * globalScale, anchor.y + slotLayout.offset.y * globalScale);

			slotLayout.size.x *= globalScale;
			slotLayout.size.y *= globalScale;

			if (slotLayout.bg_color.a > 0)
			{
				const auto [texture, width, height] = image_struct[static_cast<int32_t>(image_type::slot)];
				const auto size                     = ImVec2(slotLayout.size.x, slotLayout.size.y);
				drawElement(texture, slot_center, size, 0.f, slotLayout.bg_color);
			}

			// now draw the icon over the background...
			if (slotLayout.icon_color.a > 0)
			{
				const auto iconColor = colorizeIcons ? entry->color() : slotLayout.icon_color;
				auto iconkey         = std::string(entry->icon_key());
				if (lazyLoadIcon(iconkey))
				{
					const auto [texture, width, height] = ICON_MAP[iconkey];
					const auto scale    = width > height ? (slotLayout.icon_size.x * globalScale / width) :
					                                       (slotLayout.icon_size.y * globalScale / height);
					const auto size     = ImVec2(width * scale, height * scale);
					const auto icon_pos = ImVec2(slot_center.x + slotLayout.icon_offset.x * globalScale,
						slot_center.y + slotLayout.icon_offset.y * globalScale);

					drawElement(texture, slot_center, size, 0.f, iconColor);
				}
				else { logger::debug("lazy load for icon key {} failed; not drawing icon.", iconkey); }
			}

			// Now decide if we should draw the text showing the item's name.
			if (slotLayout.name_color.a > 0 && (entry_name.size() > 0))
			{
				const auto textPos = ImVec2(slot_center.x + slotLayout.name_offset.x * globalScale,
					slot_center.y + slotLayout.name_offset.y * globalScale);
				auto fontSize      = slotLayout.name_font_size;
				if (fontSize == 0.0) { fontSize = topLayout.font_size; }

				drawText(entry_name, textPos, fontSize * globalScale, slotLayout.name_color, slotLayout.align_text);
			}

			// Do we need to draw a count?
			if (slotLayout.count_color.a > 0 && entry->count_matters())
			{
				auto count         = entry->count();
				auto countText     = std::to_string(count);
				const auto textPos = ImVec2(slot_center.x + slotLayout.count_offset.x * globalScale,
					slot_center.y + slotLayout.count_offset.y * globalScale);

				if (!countText.empty())
				{
					drawText(countText,
						textPos,
						slotLayout.count_font_size * globalScale,
						slotLayout.count_color,
						slotLayout.align_text);
				}
			}

			if (slotLayout.hotkey_color.a > 0)
			{
				const auto hk_im_center = ImVec2(slot_center.x + slotLayout.hotkey_offset.x * globalScale,
					slot_center.y + slotLayout.hotkey_offset.y * globalScale);

				if (slotLayout.hotkey_bg_color.a > 0)
				{
					const auto [texture, width, height] = image_struct[static_cast<uint32_t>(image_type::key)];
					const auto size =
						ImVec2(slotLayout.hotkey_size.x * globalScale, slotLayout.hotkey_size.y * globalScale);
					drawElement(texture, hk_im_center, size, 0.f, slotLayout.hotkey_bg_color);
				}

				const auto [texture, width, height] = iconForHotkey(hotkey);
				const auto size = ImVec2(static_cast<float>(slotLayout.hotkey_size.x * globalScale - 2.0f),
					static_cast<float>(slotLayout.hotkey_size.y * globalScale - 2.0f));
				drawElement(texture, hk_im_center, size, 0.f, slotLayout.hotkey_color);
			}
		}

		// draw_animations_frame();
	}

	void ui_renderer::drawHud()
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
					logger::warn("file {}, does not match supported extension '.svg'"sv,
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
					logger::trace("loading texture {}, type: {}, width: {}, height: {}"sv,
						entry.path().filename().string().c_str(),
						entry.path().filename().extension().string().c_str(),
						textureCache[index].width,
						textureCache[index].height);
					*/
				}
				else { logger::error("failed to load texture {}"sv, entry.path().filename().string().c_str()); }

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
				logger::warn(
					"file {}, does not match supported extension '.svg'"sv, entry.path().filename().string().c_str());
				continue;
			}

			loadTextureFromFile(entry.path().string().c_str(), &texture, width, height);

			// logger::trace("loading animation frame: {}"sv, entry.path().string().c_str());
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
		if (a_key >= keycodes::k_gamepad_offset)
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


	void ui_renderer::loadFont()
	{
		auto hud         = hud_layout();
		auto fontfile    = std::string(hud.font);
		std::string path = R"(Data\SKSE\Plugins\resources\fonts\)" + fontfile;
		auto file_path   = std::filesystem::path(path);

		logger::trace(
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
			auto scaledSize = hud.font_size * hud.global_scale;

			imFont = io.Fonts->AddFontFromFileTTF(file_path.string().c_str(), scaledSize, nullptr, ranges.Data);
			if (io.Fonts->Build())
			{
				ImGui_ImplDX11_CreateDeviceObjects();
				logger::info("font loaded; path={}"sv, path);
				return;
			}
		}
	}

	void ui_renderer::preloadImages()
	{
		loadImagesForMap(ImageFileToType, image_struct, img_directory);
		loadImagesForMap(key_icon_name_map, key_struct, key_directory);
		loadImagesForMap(default_key_icon_name_map, default_key_struct, key_directory);
		loadImagesForMap(gamepad_ps_icon_name_map, PS5_BUTTON_MAP, key_directory);
		loadImagesForMap(gamepad_xbox_icon_name_map, XBOX_BUTTON_MAP, key_directory);

		loadAnimationFrames(highlight_animation_directory, animation_frame_map[animation_type::highlight]);
		logger::trace("frame length is {}"sv, animation_frame_map[animation_type::highlight].size());
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
		if (gDoingBriefPeek || gHudAlpha == 1.0f || (doFadeIn == true && gHudAlpha > 0.0f)) { return; }

		gDoingBriefPeek = true;
		startAlphaTransition(true, 1.0f);
	}

	void startAlphaTransition(const bool becomeVisible, const float goal)
	{
		if (becomeVisible && gHudAlpha == 1.0f) { return; }
		if (!becomeVisible && gHudAlpha == 0.0f) { return; }
		logger::debug(
			"startAlphaTransition() called with in={} and goal={}; gHudAlpha={};"sv, becomeVisible, goal, gHudAlpha);

		gGoalAlpha = std::clamp(goal, 0.0f, 1.0f);
		doFadeIn   = becomeVisible;

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
		if (helpers::hudShouldAutoFadeOut())
		{
			if (gDoingBriefPeek)
			{
				if (gHudAlpha < 1.0f) { return; }
				else { gDoingBriefPeek = false; }
			}

			if ((gHudAlpha > 0.0f && !gIsFading) || (gIsFading && doFadeIn)) { startAlphaTransition(false, 0.0f); }
		}
		else if (helpers::hudShouldAutoFadeIn())
		{
			if ((gHudAlpha < 1.0f && !gIsFading) || (gIsFading && !doFadeIn)) { startAlphaTransition(true, 1.0f); }
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
			if (gHudAlpha >= 1.0f)
			{
				gHudAlpha         = 1.0f;
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
				gHudAlpha = 1.0f - easeInCubic(1.0f - (gFadeDurRemaining / gFullFadeDuration));
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
			// logger::trace("timer decremented; timer={}; delta={}; remaining={};"sv, which, delta, remaining);
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
		logger::debug("Started equip delay timer; which={}; duration={} ms;"sv, static_cast<uint8_t>(which), duration);
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
