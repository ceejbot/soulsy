#include "include/perk_visitor.h"
#include "include/string_util.h"

namespace util
{

	RE::PerkEntryVisitor::ReturnType perk_visitor::Visit(RE::BGSPerkEntry* perk_entry)
	{
		const auto* entry_point = static_cast<RE::BGSEntryPointPerkEntry*>(perk_entry);
		const auto* perk        = entry_point->perk;

		logger::trace("form id {}, name {}"sv, string_util::int_to_hex(perk->formID), perk->GetName());

		if (entry_point->functionData)
		{
			const RE::BGSEntryPointFunctionDataOneValue* value =
				static_cast<RE::BGSEntryPointFunctionDataOneValue*>(entry_point->functionData);
			if (entry_point->entryData.function == RE::BGSEntryPointPerkEntry::EntryData::Function::kSetValue)
			{
				result_ = value->data;
			}
			else if (entry_point->entryData.function == RE::BGSEntryPointPerkEntry::EntryData::Function::kAddValue)
			{
				result_ += value->data;
			}
			else if (entry_point->entryData.function == RE::BGSEntryPointPerkEntry::EntryData::Function::kMultiplyValue)
			{
				result_ *= value->data;
			}
			else if (entry_point->entryData.function ==
					 RE::BGSEntryPointPerkEntry::EntryData::Function::kAddActorValueMult)
			{
				if (perk_entry->GetFunction() == RE::BGSPerkEntry::EntryPoint::kModPoisonDoseCount)
				{
					auto av = actor_->AsActorValueOwner()->GetActorValue(RE::ActorValue::kAlchemy);
					result_ += static_cast<float>(av * 0.1 * 3);
				}
			}

			logger::trace("Got value {} for Perk, total now is {}"sv, value->data, result_);
		}

		return ReturnType::kContinue;
	}

	float perk_visitor::get_result() const { return result_; }
}  // util
