#include "trains.h"

#include "lib.rs.h"
#include "trainwreck.h"
#include <cstdint>

struct TESForm
{
	char padding[0x30];
	std::uint32_t formID;
};

void register_with_trainwreck()
{
	trainwreck::register_section(TRAINWRECK_SECTION_MODULES,
		TRAINWRECK_SECTION_BEFORE,
		"SoulsyHUD",
		[](auto args)
		{
			auto log = trainwreck::Log(args->log_context);
			log.write_line("Relevant Soulsy data:");
			const auto crash_info = get_helpful_crash_info();
			for (const auto line : crash_info)
			{
				log.with_indent([](auto&& log) { log.write_line(line); });
			}
		});

	trainwreck::register_decoder(".?AVTESForm@@",
		[](auto args)
		{
			auto log        = trainwreck::Log(args->log_context);
			const auto form = reinterpret_cast<const TESForm*>(args->object);
			try
			{
				log.write_line(fmt::format("FormID: {:X}", form->formID));
			}
			catch (...)
			{
				// requires compiling with /EHa
				log.write_line("FormID: <INVALID>");
			}
		});
}
