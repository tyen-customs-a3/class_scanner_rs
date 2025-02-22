class CfgWeapons 
{
	class ItemCore;
	class Uniform_Base;
	class HeadGearItem;
	
	class Vest_Camo_Base: ItemCore
	{
		class ItemInfo;
	};
	
	class Binocular;
	class NVGoggles: Binocular
	{
		class ItemInfo;
	};
	
	#include "CfgWeapons_mich.hpp"
	#include "CfgWeapons_legacy.hpp"
	#include "CfgWeapons_hiddenVest.hpp"
	#include "CfgWeapons_facewear.hpp"
};