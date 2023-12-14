#pragma once

#include "animation_handler.h"
#include "image_path.h"
#include "soulsy.h"

struct LoadedImage;


namespace ui
{
	struct TextureData
	{
		ID3D11ShaderResourceView* texture = nullptr;
		int32_t width                     = 0;
		int32_t height                    = 0;
	};

	float resolutionWidth();
	float resolutionHeight();
	float resolutionScaleWidth();
	float resolutionScaleHeight();

	void drawHud();

	void makeFadeDecision();
	void showBriefly();
	void setMaxAlpha(float max);

	void startTimer(Action which, uint32_t duration);
	void stopTimer(Action which);
	void advanceTimers(float delta);
	void advanceTransition(float delta);
	void startAlphaTransition(bool a_in, float a_value);
	float easeInCubic(float progress);
	float easeOutCubic(float progress);

	void drawAllSlots();
	void drawElement(ID3D11ShaderResourceView* texture,
		const ImVec2 center,
		const ImVec2 size,
		const float angle,
		const Color color);
	void drawElementInner(ID3D11ShaderResourceView* texture,
		const ImVec2 center,
		const ImVec2 size,
		const float angle,
		const ImU32 im_color);  // retaining support for animations...
	void drawText(const std::string text,
		const ImVec2 center,
		const float font_size,
		const Color color,
		const Align alignment);
	void drawMeterCircleArc(float level, SlotFlattened slotLayout);
	void drawMeterRectangular(float level, SlotFlattened slotLayout);
	std::array<ImVec2, 4> rotateRect(const ImVec2 center, const ImVec2 size, const float angle);
	void drawTextureQuad(ID3D11ShaderResourceView* texture, const std::array<ImVec2, 4> bounds, const Color color);

	// TODO either make this use the fact that it's a class or make it not a class.
	class ui_renderer
	{
		using Color = soulsy::Color;

		struct wnd_proc_hook
		{
			static LRESULT thunk(HWND h_wnd, UINT u_msg, WPARAM w_param, LPARAM l_param);
			static inline WNDPROC func;
		};

		ui_renderer();

		// Oxidation section.
		// older...
		static void initializeAnimation(animation_type animation_type,
			float a_screen_x,
			float a_screen_y,
			float a_offset_x,
			float a_offset_y,
			float width,
			float height,
			uint32_t a_modify,
			uint32_t a_alpha,
			float a_duration);

		static bool loadTextureFromFile(const char* filename,
			ID3D11ShaderResourceView** out_srv,
			std::int32_t& out_width,
			std::int32_t& out_height);
		static bool d3dTextureFromBuffer(LoadedImage* loadedImg,
			ID3D11ShaderResourceView** out_srv,
			int32_t& out_width,
			int32_t& out_height);

		static inline ID3D11Device* device_         = nullptr;
		static inline ID3D11DeviceContext* context_ = nullptr;

		template <typename T>
		static void loadImagesForMap(std::map<std::string, T>& a_map,
			std::map<uint32_t, TextureData>& a_struct,
			std::string& file_path);

		static void loadAnimationFrames(std::string& file_path, std::vector<TextureData>& frame_list);
		static void drawAnimationFrame();

	public:
		// This only loads key/controller hotkey images.
		static void preloadImages();
		static void loadFont();
		static bool lazyLoadIcon(std::string name);
		static bool lazyLoadHudImage(std::string fname);
		static TextureData iconForHotkey(uint32_t a_key);

		struct d_3d_init_hook
		{
			static void thunk();
			static inline REL::Relocation<decltype(thunk)> func;

			static constexpr auto id     = REL::RelocationID(75595, 77226);
			static constexpr auto offset = REL::VariantOffset(0x9, 0x275, 0x00);  // VR unknown

			static inline std::atomic<bool> initialized = false;
		};

		struct dxgi_present_hook
		{
			static void thunk(std::uint32_t a_p1);
			static inline REL::Relocation<decltype(thunk)> func;

			static constexpr auto id     = REL::RelocationID(75461, 77246);
			static constexpr auto offset = REL::Offset(0x9);
		};
	};
}
