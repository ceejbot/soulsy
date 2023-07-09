#pragma once

namespace util
{
	class perk_visitor : public RE::PerkEntryVisitor
	{
	public:
		explicit perk_visitor(RE::Actor* a_actor, float a_base)
		{
			actor_  = a_actor;
			result_ = a_base;
		}

		ReturnType Visit(RE::BGSPerkEntry* perk_entry) override;

		[[nodiscard]] float get_result() const;

	protected:
		RE::Actor* actor_;
		float result_;
	};
}  // util
