ScriptName SoulsyHUD_Decorators Extends ReferenceAlias

import SKI_ConfigBase
import AhzMoreHudIE

bool property bMoreHUDFound = false auto
ReferenceAlias property pPlayerRefAlias auto

event OnPlayerLoadGame()
	bMoreHUDFound = SKSE.GetPluginVersion("Ahzaab's moreHUD Inventory Plugin") > 0
	if bMoreHUDFound
		self.RegisterForModEvent("Soulsy_ItemAddedToCycle", "handleItemAddedToCycle")
		self.RegisterForModEvent("Soulsy_ItemRemovedFromCycle", "handleItemRemovedFromCycle")
	endif
endEvent

event handleItemAddedToCycle(string eventName, string decorator, float unused, Form item)
	int itemID = GetFormItemId(item)

	;if AhzMoreHudIE.IsIconItemRegistered(itemID)
	;	AhzMoreHudIE.RemoveIconItem(itemID)
	; endIf
	AddIconItem(itemID, decorator)
endEvent

event handleItemRemovedFromCycle(string eventName, string unused, float unused2, Form item)
	int itemID = AhzMoreHudIE.GetFormItemId(item)
	if AhzMoreHudIE.IsIconItemRegistered(itemID)
		AhzMoreHudIE.RemoveIconItem(itemID)
	endIf
endEvent
