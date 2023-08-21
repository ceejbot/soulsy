#include "ui_renderer.h"
#include "animation_handler.h"
#include "constant.h"
#include "gear.h"
#include "helpers.h"
#include "image_path.h"
#include "key_path.h"
#include "keycodes.h"

#pragma warning(push)
#pragma warning(disable : 4702)
#define NANOSVG_IMPLEMENTATION
#define NANOSVG_ALL_COLOR_KEYWORDS
#include <nanosvg.h>
#define NANOSVGRAST_IMPLEMENTATION
#include <nanosvgrast.h>
#pragma warning(pop)

#include "lib.rs.h"

namespace ui
{
	static std::map<animation_type, std::vector<image>> animation_frame_map = {};
	static std::vector<std::pair<animation_type, std::unique_ptr<animation>>> animation_list;

	static std::map<uint8_t, float> cycle_timers = {};

	static std::map<uint32_t, image> image_struct;
	static std::map<uint32_t, image> key_struct;
	static std::map<uint32_t, image> default_key_struct;
	static std::map<uint32_t, image> ps_key_struct;
	static std::map<uint32_t, image> xbox_key_struct;
	static std::map<std::string, image> icon_struct;

	static const float FADEOUT_HYSTERESIS = 0.5f;  // seconds

	auto hud_alpha        = 0.0f;
	auto goal_alpha       = 1.0f;
	auto fade_in          = true;
	auto fade_duration    = 3.0f;  // seconds
	auto transition_timer = 2.0f;  // seconds
	auto is_transitioning = false;
	auto fade_out_timer   = 0.33f;  // seconds
	bool doing_brief_peek = false;

	ImFont* loaded_font;
	auto tried_font_load = false;

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
		const auto render_manager = RE::BSRenderManager::GetSingleton();
		if (!render_manager)
		{
			logger::error("Cannot find game render manager. Initialization failed."sv);
			return;
		}

		const auto [forwarder, context, unk58, unk60, unk68, swapChain, unk78, unk80, renderView, resourceView] =
			render_manager->GetRuntimeData();

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

		// Not the right solution, but noting here anyway: to get alpha blending, we need to
		// enable it in the ID3D11Device. I think. But this is not the correct call for the
		// version we get from the call? or something. TODO: learn more dx11 trivia.
		// device_->OMSetBlendState(true, GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA, GL_FUNC_ADD);

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
		logger::info("Ready to render.");

		wnd_proc_hook::func = reinterpret_cast<WNDPROC>(
			SetWindowLongPtrA(sd.OutputWindow, GWLP_WNDPROC, reinterpret_cast<LONG_PTR>(wnd_proc_hook::thunk)));
		if (!wnd_proc_hook::func) { logger::error("SetWindowLongPtrA failed"sv); }
	}

	void ui_renderer::dxgi_present_hook::thunk(std::uint32_t a_p1)
	{
		func(a_p1);

		if (!d_3d_init_hook::initialized.load()) { return; }

		if (!loaded_font && !tried_font_load) { load_font(); }

		ImGui_ImplDX11_NewFrame();
		ImGui_ImplWin32_NewFrame();
		ImGui::NewFrame();

		draw_ui();

		ImGui::EndFrame();
		ImGui::Render();
		ImGui_ImplDX11_RenderDrawData(ImGui::GetDrawData());
	}

	// Simple helper function to load an image into a DX11 texture with common settings
	bool ui_renderer::load_texture_from_file(const char* filename,
		ID3D11ShaderResourceView** out_srv,
		int32_t& out_width,
		int32_t& out_height)
	{
		auto* render_manager = RE::BSRenderManager::GetSingleton();
		if (!render_manager)
		{
			logger::error("Cannot find render manager. Initialization failed."sv);
			return false;
		}

		auto [forwarder, context, unk58, unk60, unk68, swapChain, unk78, unk80, renderView, resourceView] =
			render_manager->GetRuntimeData();

		// Load from disk into a raw RGBA buffer
		auto* svg  = nsvgParseFromFile(filename, "px", 96.0f);
		auto* rast = nsvgCreateRasterizer();

		auto image_width  = static_cast<int>(svg->width);
		auto image_height = static_cast<int>(svg->height);

		auto image_data = (unsigned char*)malloc(image_width * image_height * 4);
		nsvgRasterize(rast, svg, 0, 0, 1, image_data, image_width, image_height, image_width * 4);
		nsvgDelete(svg);
		nsvgDeleteRasterizer(rast);

		// Create texture
		D3D11_TEXTURE2D_DESC desc;
		ZeroMemory(&desc, sizeof(desc));
		desc.Width            = image_width;
		desc.Height           = image_height;
		desc.MipLevels        = 1;
		desc.ArraySize        = 1;
		desc.Format           = DXGI_FORMAT_R8G8B8A8_UNORM;
		desc.SampleDesc.Count = 1;
		desc.Usage            = D3D11_USAGE_DEFAULT;
		desc.BindFlags        = D3D11_BIND_SHADER_RESOURCE;
		desc.CPUAccessFlags   = 0;
		desc.MiscFlags        = 0;

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

		out_width  = image_width;
		out_height = image_height;

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
				draw_element(animation_frame_map[it->first][anim->current_frame].texture,
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

		const ImU32 text_color   = IM_COL32(color.r, color.g, color.b, color.a * hud_alpha);
		const ImVec2 text_bounds = ImGui::CalcTextSize(text.c_str());
		auto* font               = loaded_font;
		if (!font) { font = ImGui::GetDefaultFont(); }

		// It's left-aligned by default.
		float adjustment = 0;
		if (align == Align::Center) { adjustment = -0.25f * text_bounds.x; }
		else if (align == Align::Right) { adjustment = -1.25f * text_bounds.x; }

		ImVec2 aligned_center = ImVec2(center.x + adjustment, center.y);

		ImGui::GetWindowDrawList()->AddText(
			font, font_size, aligned_center, text_color, text.c_str(), nullptr, 0.0f, nullptr);
	}


	void ui_renderer::draw_text(const float a_x,
		const float a_y,
		const float a_offset_x,
		const float a_offset_y,
		const float a_offset_extra_x,
		const float a_offset_extra_y,
		const char* a_text,
		uint32_t a_alpha,
		uint32_t a_red,
		uint32_t a_green,
		uint32_t a_blue,
		const float a_font_size,
		bool a_center_text,
		bool a_deduct_text_x,
		bool a_deduct_text_y,
		bool a_add_text_x,
		bool a_add_text_y)
	{
		//it should center the text, it kind of does
		auto text_x = 0.f;
		auto text_y = 0.f;

		if (!a_text || !*a_text || a_alpha == 0) { return; }

		const ImU32 color = IM_COL32(a_red, a_green, a_blue, a_alpha * hud_alpha);

		const ImVec2 text_size = ImGui::CalcTextSize(a_text);
		if (a_center_text)
		{
			text_x = -text_size.x * 0.5f;
			text_y = -text_size.y * 0.5f;
		}
		if (a_deduct_text_x) { text_x = text_x - text_size.x; }
		if (a_deduct_text_y) { text_y = text_y - text_size.y; }
		if (a_add_text_x) { text_x = text_x + text_size.x; }
		if (a_add_text_y) { text_y = text_y + text_size.y; }

		const auto position =
			ImVec2(a_x + a_offset_x + a_offset_extra_x + text_x, a_y + a_offset_y + a_offset_extra_y + text_y);

		auto* font = loaded_font;
		if (!font) { font = ImGui::GetDefaultFont(); }

		ImGui::GetWindowDrawList()->AddText(font, a_font_size, position, color, a_text, nullptr, 0.0f, nullptr);
	}

	// Used only by draw_animations_frame
	void ui_renderer::draw_element(ID3D11ShaderResourceView* a_texture,
		const ImVec2 a_center,
		const ImVec2 a_size,
		const float a_angle,
		const ImU32 a_color)
	{
		const float cos_a   = cosf(a_angle);
		const float sin_a   = sinf(a_angle);
		const ImVec2 pos[4] = { a_center + ImRotate(ImVec2(-a_size.x * 0.5f, -a_size.y * 0.5f), cos_a, sin_a),
			a_center + ImRotate(ImVec2(+a_size.x * 0.5f, -a_size.y * 0.5f), cos_a, sin_a),
			a_center + ImRotate(ImVec2(+a_size.x * 0.5f, +a_size.y * 0.5f), cos_a, sin_a),
			a_center + ImRotate(ImVec2(-a_size.x * 0.5f, +a_size.y * 0.5f), cos_a, sin_a)

		};
		constexpr ImVec2 uvs[4] = { ImVec2(0.0f, 0.0f), ImVec2(1.0f, 0.0f), ImVec2(1.0f, 1.0f), ImVec2(0.0f, 1.0f) };

		ImGui::GetWindowDrawList()->AddImageQuad(
			a_texture, pos[0], pos[1], pos[2], pos[3], uvs[0], uvs[1], uvs[2], uvs[3], a_color);
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

		logger::trace("starting inited animation");
		constexpr auto angle = 0.0f;

		const auto size = static_cast<uint32_t>(animation_frame_map[animation_type].size());
		// const auto width  = static_cast<uint32_t>(animation_frame_map[animation_type][0].width);
		// const auto height = static_cast<uint32_t>(animation_frame_map[animation_type][0].height);

		std::unique_ptr<animation> anim =
			std::make_unique<fade_framed_out_animation>(ImVec2(a_screen_x + a_offset_x, a_screen_y + a_offset_y),
				ImVec2(width, height),
				angle,
				a_alpha,
				a_modify,
				a_modify,
				a_modify,
				a_duration,
				size);
		animation_list.emplace_back(static_cast<ui::animation_type>(animation_type), std::move(anim));
		logger::trace("done initializing animation. return.");
	}

	void ui_renderer::drawElement(ID3D11ShaderResourceView* texture,
		const ImVec2 center,
		const ImVec2 size,
		const float angle,
		const Color color)
	{
		const ImU32 im_color = IM_COL32(color.r, color.g, color.b, color.a * hud_alpha);

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
		auto top_layout         = hud_layout();
		auto anchor             = top_layout.anchor;
		auto hudsize            = top_layout.size;
		bool rangedEquipped     = player::hasRangedEquipped();
		const auto settings     = user_settings();
		const auto screenWidth  = get_resolution_width();
		const auto screenHeight = get_resolution_height();
		bool colorizeIcons      = settings->colorize_icons();

		auto globalScale = top_layout.global_scale;
		if (globalScale == 0.0f)
		{
			globalScale = 1.0f;  // serde's default for missing f32 fields is 0
		}

		// If the layout is larger than the HUD, clamp it to screen size.
		hudsize.x = std::min(screenWidth, globalScale * hudsize.x);
		hudsize.y = std::min(screenHeight, globalScale * hudsize.y);

		// If the layout is trying to draw the HUD offscreen, clamp it to an edge.
		anchor.x = std::max(hudsize.x / 2.0f, anchor.x);  // anchor point is center
		anchor.x = std::min(screenWidth - hudsize.x / 2.0f, anchor.x);
		anchor.y = std::max(hudsize.y / 2.0f, anchor.y);
		anchor.y = std::min(screenHeight - hudsize.y / 2.0f, anchor.y);


		// Draw the HUD background if requested.
		if (top_layout.bg_color.a > 0)
		{
			constexpr auto angle                = 0.f;
			const auto center                   = ImVec2(anchor.x, anchor.y);
			const auto [texture, width, height] = image_struct[static_cast<int32_t>(image_type::hud)];
			const auto size                     = ImVec2(hudsize.x, hudsize.y);
			drawElement(texture, center, size, angle, top_layout.bg_color);
		}

		for (auto slot_layout : top_layout.layouts)
		{
			if ((slot_layout.element == HudElement::Left) && top_layout.hide_left_when_irrelevant && rangedEquipped)
			{
				continue;
			}
			if ((slot_layout.element == HudElement::Ammo) && top_layout.hide_ammo_when_irrelevant && !rangedEquipped)
			{
				continue;
			}

			rust::Box<HudItem> entry = entry_to_show_in_slot(slot_layout.element);

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

			const auto hotkey = settings->hotkey_for(slot_layout.element);
			const auto slot_center =
				ImVec2(anchor.x + slot_layout.offset.x * globalScale, anchor.y + slot_layout.offset.y * globalScale);

			slot_layout.size.x *= globalScale;
			slot_layout.size.y *= globalScale;

			if (slot_layout.bg_color.a > 0)
			{
				const auto [texture, width, height] = image_struct[static_cast<int32_t>(image_type::slot)];
				const auto size                     = ImVec2(slot_layout.size.x, slot_layout.size.y);
				drawElement(texture, slot_center, size, 0.f, slot_layout.bg_color);
			}

			// now draw the icon over the background...
			if (slot_layout.icon_color.a > 0)
			{
				const auto iconColor                = colorizeIcons ? entry->color() : slot_layout.icon_color;
				auto iconFile                       = std::string(entry->icon_file());
				const auto [texture, width, height] = icon_struct[iconFile];
				const auto scale                    = width > height ? (slot_layout.icon_size.x * globalScale / width) :
				                                                       (slot_layout.icon_size.y * globalScale / height);
				const auto size                     = ImVec2(width * scale, height * scale);
				const auto icon_pos                 = ImVec2(slot_center.x + slot_layout.icon_offset.x * globalScale,
                    slot_center.y + slot_layout.icon_offset.y * globalScale);

				drawElement(texture, slot_center, size, 0.f, iconColor);
			}

			// Now decide if we should draw the text showing the item's name.
			if (slot_layout.name_color.a > 0 && (entry_name.size() > 0))
			{
				const auto textPos = ImVec2(slot_center.x + slot_layout.name_offset.x * globalScale,
					slot_center.y + slot_layout.name_offset.y * globalScale);
				auto fontSize      = slot_layout.name_font_size;
				if (fontSize == 0.0) { fontSize = top_layout.font_size; }

				drawText(entry_name, textPos, fontSize * globalScale, slot_layout.name_color, slot_layout.align_text);
			}

			// Do we need to draw a count?
			if (slot_layout.count_color.a > 0 && entry->count_matters())
			{
				auto count         = entry->count();
				auto countText     = std::to_string(count);
				const auto textPos = ImVec2(slot_center.x + slot_layout.count_offset.x * globalScale,
					slot_center.y + slot_layout.count_offset.y * globalScale);

				if (!countText.empty())
				{
					drawText(countText,
						textPos,
						slot_layout.count_font_size * globalScale,
						slot_layout.count_color,
						slot_layout.align_text);
				}
			}

			if (slot_layout.hotkey_color.a > 0)
			{
				const auto hk_im_center = ImVec2(slot_center.x + slot_layout.hotkey_offset.x * globalScale,
					slot_center.y + slot_layout.hotkey_offset.y * globalScale);

				if (slot_layout.hotkey_bg_color.a > 0)
				{
					const auto [texture, width, height] = image_struct[static_cast<uint32_t>(image_type::key)];
					const auto size =
						ImVec2(slot_layout.hotkey_size.x * globalScale, slot_layout.hotkey_size.y * globalScale);
					drawElement(texture, hk_im_center, size, 0.f, slot_layout.hotkey_bg_color);
				}

				const auto [texture, width, height] = get_key_icon(hotkey);
				const auto size = ImVec2(static_cast<float>(slot_layout.hotkey_size.x * globalScale - 2.0f),
					static_cast<float>(slot_layout.hotkey_size.y * globalScale - 2.0f));
				drawElement(texture, hk_im_center, size, 0.f, slot_layout.hotkey_color);
			}
		}

		// draw_animations_frame();
	}

	void ui_renderer::draw_ui()
	{
		const auto timeDelta = ImGui::GetIO().DeltaTime;
		advanceTimers(timeDelta);
		const auto settings = user_settings();

		if (!helpers::hudAllowedOnScreen()) return;
		makeFadeDecision();
		advanceTransition(timeDelta);
		if (hud_alpha == 0.0f) { return; }

		static constexpr ImGuiWindowFlags window_flags =
			ImGuiWindowFlags_NoBackground | ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_NoInputs;

		const float screen_size_x = ImGui::GetIO().DisplaySize.x, screen_size_y = ImGui::GetIO().DisplaySize.y;

		ImGui::SetNextWindowSize(ImVec2(screen_size_x, screen_size_y));
		ImGui::SetNextWindowPos(ImVec2(0.f, 0.f));
		ImGui::GetStyle().Alpha = hud_alpha;
		ImGui::Begin(hud_name, nullptr, window_flags);

		drawAllSlots();

		ImGui::End();
	}

	void ui_renderer::load_icon_images(std::map<std::string, image>& out_struct, std::string& icondir)
	{
		const auto res_width = 1.0f;
		get_resolution_scale_width();
		const auto res_height = 1.0f;
		get_resolution_scale_height();

		const auto needed_icons = icon_files();
		for (auto icon_file_str : needed_icons)
		{
			auto icon_file = std::string(icon_file_str);
			auto entrypath = std::filesystem::path(icondir);
			entrypath /= icon_file;

			std::error_code ec;
			if (std::filesystem::exists(entrypath, ec))
			{
				if (load_texture_from_file(entrypath.string().c_str(),
						&out_struct[icon_file].texture,
						out_struct[icon_file].width,
						out_struct[icon_file].height))
				{
					logger::debug("loading texture {}, type: {}, width: {}, height: {}"sv,
						entrypath.filename().string().c_str(),
						entrypath.filename().extension().string().c_str(),
						out_struct[icon_file].width,
						out_struct[icon_file].height);

					out_struct[icon_file].width  = static_cast<int32_t>(out_struct[icon_file].width * res_width);
					out_struct[icon_file].height = static_cast<int32_t>(out_struct[icon_file].height * res_height);
				}
			}
			else { logger::error("failed to load {}"sv, entrypath.filename().string().c_str()); }
		}
	}

	template <typename T>
	void ui_renderer::load_images(std::map<std::string, T>& a_map,
		std::map<uint32_t, image>& a_struct,
		std::string& file_path)
	{
		const auto res_width = 1.0f;
		get_resolution_scale_width();
		const auto res_height = 1.0f;
		get_resolution_scale_height();

		for (const auto& entry : std::filesystem::directory_iterator(file_path))
		{
			if (a_map.contains(entry.path().filename().string()))
			{
				if (entry.path().filename().extension() != ".svg")
				{
					logger::warn("file {}, does not match supported extension '.svg'"sv,
						entry.path().filename().string().c_str());
					continue;
				}
				const auto index = static_cast<int32_t>(a_map[entry.path().filename().string()]);
				if (load_texture_from_file(entry.path().string().c_str(),
						&a_struct[index].texture,
						a_struct[index].width,
						a_struct[index].height))
				{
					/*
					logger::trace("loading texture {}, type: {}, width: {}, height: {}"sv,
						entry.path().filename().string().c_str(),
						entry.path().filename().extension().string().c_str(),
						a_struct[index].width,
						a_struct[index].height);
						*/
				}
				else { logger::error("failed to load texture {}"sv, entry.path().filename().string().c_str()); }

				a_struct[index].width  = static_cast<int32_t>(a_struct[index].width * res_width);
				a_struct[index].height = static_cast<int32_t>(a_struct[index].height * res_height);
			}
		}
	}

	void ui_renderer::load_animation_frames(std::string& file_path, std::vector<image>& frame_list)
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

			load_texture_from_file(entry.path().string().c_str(), &texture, width, height);

			logger::trace("loading animation frame: {}"sv, entry.path().string().c_str());
			image img;
			img.texture = texture;
			// img.width   = static_cast<int32_t>(width * get_resolution_scale_width());
			// img.height  = static_cast<int32_t>(height * get_resolution_scale_height());
			frame_list.push_back(img);
		}
	}

	image ui_renderer::get_key_icon(const uint32_t a_key)
	{
		const auto settings = user_settings();
		auto return_image   = default_key_struct[static_cast<int32_t>(default_keys::key)];
		// todo rework this logic at some point, no rush
		if (a_key >= keycodes::k_gamepad_offset)
		{
			if (settings->controller_kind() == static_cast<uint32_t>(controller_set::playstation))
			{
				return_image = ps_key_struct[a_key];
			}
			else { return_image = xbox_key_struct[a_key]; }
		}
		else
		{
			if (key_struct.contains(a_key)) { return_image = key_struct[a_key]; }
		}
		return return_image;
	}

	// but y tho?
	// float ui_renderer::get_resolution_scale_width() { return ImGui::GetIO().DisplaySize.x / 1920.f; }
	// float ui_renderer::get_resolution_scale_height() { return ImGui::GetIO().DisplaySize.y / 1080.f; }

	float ui_renderer::get_resolution_scale_width() { return 1.0f; }
	float ui_renderer::get_resolution_scale_height() { return 1.0f; }

	float ui_renderer::get_resolution_width() { return ImGui::GetIO().DisplaySize.x; }
	float ui_renderer::get_resolution_height() { return ImGui::GetIO().DisplaySize.y; }

	void ui_renderer::show_briefly()
	{
		if (doing_brief_peek || hud_alpha == 1.0f || (fade_in == true && hud_alpha > 0.0f)) { return; }

		doing_brief_peek = true;
		ui_renderer::startAlphaTransition(true, 1.0f);
	}

	void ui_renderer::startAlphaTransition(const bool a_in, const float a_value)
	{
		if (a_in && hud_alpha == 1.0f) { return; }
		if (!a_in && hud_alpha == 0.0f) { return; }
		logger::debug(
			"startAlphaTransition() called with in={} and goal={}; hud_alpha={};"sv, a_in, a_value, hud_alpha);
		is_transitioning = true;
		fade_in          = a_in;

		// unused right now
		if (a_value < 0) { goal_alpha = 0.0; }
		else if (a_value > 1.0) { goal_alpha = 1.0; }
		else { goal_alpha = a_value; }

		// The game will report that the player has sheathed weapons when
		// the player has merely equipped something new. So we give it some
		// time to decide that the weapons are truly gone.
		fade_out_timer = FADEOUT_HYSTERESIS;

		auto settings   = user_settings();
		float fade_time = static_cast<float>(settings->fade_time()) / 1000.0f;
		if (doing_brief_peek)
		{
			fade_time += static_cast<float>(settings->equip_delay_ms()) / 250.0f;  // yes we're waiting longer
		}
		transition_timer = fade_in ? (fade_time / 2.0f) : fade_time;  // fade in is faster than fade out

		// We must allow for the transition starting while the alpha is not pinned.
		// Scale the transition time for how much of the shift remains.
		if (fade_in) { fade_duration = 1.0f - hud_alpha * transition_timer; }
		else { fade_duration = hud_alpha * transition_timer; }
	}

	void ui_renderer::makeFadeDecision()
	{
		if (helpers::hudShouldAutoFadeOut())
		{
			if (doing_brief_peek)
			{
				if (hud_alpha < 1.0f) { return; }
				else { doing_brief_peek = false; }
			}

			if ((hud_alpha > 0.0f && !is_transitioning) || (is_transitioning && fade_in))
			{
				startAlphaTransition(false, 0.0f);
			}
		}
		else if (helpers::hudShouldAutoFadeIn())
		{
			if ((hud_alpha < 1.0f && !is_transitioning) || (is_transitioning && !fade_in))
			{
				startAlphaTransition(true, 1.0f);
			}
		}
	}

	float ui_renderer::easeInCubic(float progress)
	{
		if (progress >= 1.0f) return 1.0f;
		if (progress <= 0.0f) return 0.0f;
		return static_cast<float>(pow(progress, 3));
	}

	float ui_renderer::easeOutCubic(float progress)
	{
		if (progress >= 1.0f) return 1.0f;
		if (progress <= 0.0f) return 0.0f;
		return static_cast<float>(1.0f - pow(1 - progress, 3));
	}

	void ui_renderer::advanceTransition(float timeDelta)
	{
		if (fade_in && is_transitioning)
		{
			if (hud_alpha >= 1.0f)
			{
				hud_alpha        = 1.0f;
				transition_timer = 0.0f;
				is_transitioning = false;
				return;
			}
			if (transition_timer > 0.0f) { transition_timer -= timeDelta; }
			hud_alpha = ui_renderer::easeInCubic(1.0f - (transition_timer / fade_duration));
		}
		else if (!fade_in && is_transitioning)
		{
			if (fade_out_timer > 0.0f) { fade_out_timer -= timeDelta; }
			else
			{
				if (hud_alpha <= 0.0f)
				{
					hud_alpha        = 0.0f;
					transition_timer = 0.0f;
					is_transitioning = false;
				}
				fade_out_timer = 0.0f;
				if (transition_timer > 0.0f) { transition_timer -= timeDelta; }
				hud_alpha = 1.0f - ui_renderer::easeInCubic(1.0f - (transition_timer / fade_duration));
			}
		}
	}

	void ui_renderer::load_font()
	{
		auto hud         = hud_layout();
		auto fontfile    = std::string(hud.font);
		std::string path = R"(Data\SKSE\Plugins\resources\fonts\)" + fontfile;
		auto file_path   = std::filesystem::path(path);

		logger::trace("about to try to load font; path={}"sv, path);
		tried_font_load = true;
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

			loaded_font = io.Fonts->AddFontFromFileTTF(file_path.string().c_str(), hud.font_size, nullptr, ranges.Data);
			if (io.Fonts->Build())
			{
				ImGui_ImplDX11_CreateDeviceObjects();
				logger::info("font loaded; path={}"sv, path);
				return;
			}
		}
	}

	// TODO ceej: rewrite in rust
	void ui_renderer::load_all_images()
	{
		load_images(image_type_name_map, image_struct, img_directory);
		load_images(key_icon_name_map, key_struct, key_directory);
		load_images(default_key_icon_name_map, default_key_struct, key_directory);
		load_images(gamepad_ps_icon_name_map, ps_key_struct, key_directory);
		load_images(gamepad_xbox_icon_name_map, xbox_key_struct, key_directory);

		load_icon_images(icon_struct, icon_directory);

		load_animation_frames(highlight_animation_directory, animation_frame_map[animation_type::highlight]);
		logger::trace("frame length is {}"sv, animation_frame_map[animation_type::highlight].size());
	}

	// We implement timers using UI ticks. We don't need them to be
	// particularly accurate, just good-feeling to humans. Because we only
	// manage timers here, this is the right decision point for going into
	// and out of slow motion.
	void ui_renderer::advanceTimers(float delta)
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

	void ui_renderer::startTimer(Action which)
	{
		// We replace any existing timer for this slot.
		auto settings = user_settings();
		auto duration = settings->equip_delay_ms();  // this is in ms, so we'll divide...
		cycle_timers.insert_or_assign(static_cast<uint8_t>(which), static_cast<float>(duration) / 1000.0f);
		logger::info("started equip delay timer; which={}; delay={};"sv,
			static_cast<uint8_t>(which),
			static_cast<float>(duration) / 1000.0f);
		if (settings->cycling_slows_time() && RE::PlayerCharacter::GetSingleton()->IsInCombat())
		{
			helpers::enterSlowMotion();
		}
	}

	// remove timer from the map if it exists
	void ui_renderer::stopTimer(Action which)
	{
		cycle_timers.erase(static_cast<uint8_t>(which));
		if (cycle_timers.size() == 0) { helpers::exitSlowMotion(); }
	}
}
