class pca_nvg_dummy_base: NVGoggles
{
	scope = 1;
	author = "pca";
	descriptionShort = "No Armor";
	modelOptics = "\A3\Weapons_F\empty";
	visionMode[] = 
	{
		"Normal",
		"Normal"
	};
};

class pca_nvg_balaclava: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Balaclava (Olive)";
	picture = "\rhsafrf\addons\rhs_infantry\Data\inventory\gear_icon_balaclava_ca.paa";
	model = "\rhsafrf\addons\rhs_infantry2\gear\head\rhs_bala1";
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		uniformModel = "\rhsafrf\addons\rhs_infantry2\gear\head\rhs_bala1";
		modelOff = "\rhsafrf\addons\rhs_infantry2\gear\head\rhs_bala1";
		mass = 4;
	};
};

class pca_nvg_balaclava2: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Balaclava 2 (Olive)";
	picture = "\rhsafrf\addons\rhs_infantry\Data\inventory\gear_icon_balaclava_ca.paa";
	model = "\rhsafrf\addons\rhs_infantry2\gear\head\rhs_bala2";
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		uniformModel = "\rhsafrf\addons\rhs_infantry2\gear\head\rhs_bala2";
		modelOff = "\rhsafrf\addons\rhs_infantry2\gear\head\rhs_bala2";
		mass = 4;
	};
};

class pca_nvg_ess_blk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Ballistic Goggles (Black)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\rhsusf_ess_black_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_ess_black";
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		uniformModel = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_ess_black";
		modelOff = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_ess_black";
		mass = 8;
	};
};

class pca_nvg_glasses_blk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Ballistic Glasses (Black)";
	picture = "\rhsusf\addons\rhsusf_infantry\ui\ballisticglasses_black_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_goggles_black";
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		uniformModel = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_goggles_black";
		modelOff = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_goggles_black";
		mass = 4;
	};
};

class pca_nvg_glasses_clr: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Ballistic Glasses (Clear)";
	picture = "\rhsusf\addons\rhsusf_infantry\ui\ballisticglasses_clear_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_goggles_clear";
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		uniformModel = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_goggles_clear";
		modelOff = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_goggles_clear";
		mass = 4;
	};
};

class pca_nvg_glasses_org: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Ballistic Glasses (Orange)";
	picture = "\rhsusf\addons\rhsusf_infantry\ui\ballisticglasses_orange_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_goggles_orange";
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		uniformModel = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_goggles_orange";
		modelOff = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_goggles_orange";
		mass = 4;
	};
};

class pca_nvg_glasses_ylw: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Ballistic Glasses (Yellow)";
	picture = "\rhsusf\addons\rhsusf_infantry\ui\ballisticglasses_yellow_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_goggles_yellow";
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		uniformModel = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_goggles_yellow";
		modelOff = "\rhsusf\addons\rhsusf_infantry\gear\head\rhsusf_goggles_yellow";
		mass = 4;
	};
};

class pca_nvg_oakley_goggles_blk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "SI Ballistic 2.0 (Black)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesb_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry2\gear\head\goggles\oakley";
	hiddenSelections[] =
	{
		"Camo2",
		"Camo3"
	};
	hiddenSelectionsTextures[] =
	{
		"rhsusf\addons\rhsusf_infantry2\gear\head\goggles\data\dark_ca.paa",
		"rhsusf\addons\rhsusf_infantry2\gear\head\goggles\data\oakleysblk_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"Camo2",
			"Camo3"
		};
		uniformModel = "\rhsusf\addons\rhsusf_infantry2\gear\head\goggles\oakley";
		modelOff = "\rhsusf\addons\rhsusf_infantry2\gear\head\goggles\oakley";
		mass = 8;
	};
};

class pca_nvg_oakley_goggles_clr: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "SI Ballistic 2.0 (Clear)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry2\gear\head\goggles\oakley";
	hiddenSelections[] =
	{
		"Camo2",
		"Camo3"
	};
	hiddenSelectionsTextures[] =
	{
		"rhsusf\addons\rhsusf_infantry2\gear\head\goggles\data\glass_ca.paa",
		"rhsusf\addons\rhsusf_infantry2\gear\head\goggles\data\oakleysblk_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"Camo2",
			"Camo3"
		};
		uniformModel = "\rhsusf\addons\rhsusf_infantry2\gear\head\goggles\oakley";
		modelOff = "\rhsusf\addons\rhsusf_infantry2\gear\head\goggles\oakley";
		mass = 8;
	};
};

class pca_nvg_oakley_goggles_ylw: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "SI Ballistic 2.0 (Yellow)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesy_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry2\gear\head\goggles\oakley";
	hiddenSelections[] =
	{
		"Camo2",
		"Camo3"
	};
	hiddenSelectionsTextures[] =
	{
		"rhsusf\addons\rhsusf_infantry2\gear\head\goggles\data\yellow_ca.paa",
		"rhsusf\addons\rhsusf_infantry2\gear\head\goggles\data\oakleysblk_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"Camo2",
			"Camo3"
		};
		uniformModel = "\rhsusf\addons\rhsusf_infantry2\gear\head\goggles\oakley";
		modelOff = "\rhsusf\addons\rhsusf_infantry2\gear\head\goggles\oakley";
		mass = 8;
	};
};

class pca_nvg_shemagh_grn: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh (Green)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\shemagholive_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh";
	hiddenSelections[] =
	{
		"Camo"
	};
	hiddenSelectionsTextures[] =
	{
		"rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\data\shemagh_grn_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"Camo"
		};
		uniformModel = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh";
		modelOff = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh";
		mass = 4;
	};
};

class pca_nvg_shemagh_od: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh (OD)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\shemagholive_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh";
	hiddenSelections[] =
	{
		"Camo"
	};
	hiddenSelectionsTextures[] =
	{
		"rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\data\shemagh_od_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"Camo"
		};
		uniformModel = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh";
		modelOff = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh";
		mass = 4;
	};
};

class pca_nvg_shemagh_tan: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh (Tan)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\shemaghtan_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh";
	hiddenSelections[] =
	{
		"Camo"
	};
	hiddenSelectionsTextures[] =
	{
		"rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\data\shemagh_tan_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"Camo"
		};
		uniformModel = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh";
		modelOff = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh";
		mass = 4;
	};
};

class pca_nvg_shemagh_white: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh (White)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\shemaghwhite_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh";
	hiddenSelections[] =
	{
		"Camo"
	};
	hiddenSelectionsTextures[] =
	{
		"rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\data\shemagh_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"Camo"
		};
		uniformModel = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh";
		modelOff = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh";
		mass = 4;
	};
};

class pca_nvg_shemagh2_grn: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh (Green/Alt)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\shemagholive_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh2";
	hiddenSelections[] =
	{
		"Camo"
	};
	hiddenSelectionsTextures[] =
	{
		"rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\data\shemagh2_grn_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"Camo"
		};
		uniformModel = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh2";
		modelOff = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh2";
		mass = 4;
	};
};

class pca_nvg_shemagh2_od: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh (OD/Alt)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\shemagholive_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh2";
	hiddenSelections[] =
	{
		"Camo"
	};
	hiddenSelectionsTextures[] =
	{
		"rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\data\shemagh2_od_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"Camo"
		};
		uniformModel = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh2";
		modelOff = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh2";
		mass = 4;
	};
};

class pca_nvg_shemagh2_tan: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh (Tan/Alt)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\shemaghtan_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh2";
	hiddenSelections[] =
	{
		"Camo"
	};
	hiddenSelectionsTextures[] =
	{
		"rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\data\shemagh2_tan_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"Camo"
		};
		uniformModel = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh2";
		modelOff = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh2";
		mass = 4;
	};
};

class pca_nvg_shemagh2_white: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh (White/Alt)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\shemaghwhite_ca.paa";
	model = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh2";
	hiddenSelections[] =
	{
		"Camo"
	};
	hiddenSelectionsTextures[] =
	{
		"rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\data\shemagh2_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"Camo"
		};
		uniformModel = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh2";
		modelOff = "\rhsusf\addons\rhsusf_infantry2\gear\head\shemagh\shemagh2";
		mass = 4;
	};
};

class pca_nvg_tactical_glasses: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Tactical Glasses";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"_Shades",
		"_ESS",
		"_Shemagh"
	};
	hiddenSelectionsTextures[] = {};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"_Shades",
			"_ESS",
			"_Shemagh"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 4;
	};
};

class pca_nvg_tactical_goggles: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Tactical Goggles";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"_Tactical",
		"_Shades",
		"_Shemagh"
	};
	hiddenSelectionsTextures[] = {};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"_Tactical",
			"_Shades",
			"_Shemagh"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 8;
	};
};

class pca_nvg_face_shield_blk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Face Shield (Black)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"_Tactical",
		"_Shades",
		"_ESS",
		"_Shemagh"
	};
	hiddenSelectionsTextures[] = 
	{
		"milgp_headgear\textures\f_face_shield_blk_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"_Tactical",
			"_Shades",
			"_ESS",
			"_Shemagh"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 4;
	};
};

class pca_nvg_face_shield_cb: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Face Shield (Coyote Brown)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"_Tactical",
		"_Shades",
		"_ESS",
		"_Shemagh"
	};
	hiddenSelectionsTextures[] = 
	{
		"milgp_headgear\textures\f_face_shield_cb_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"_Tactical",
			"_Shades",
			"_ESS",
			"_Shemagh"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 4;
	};
};

class pca_nvg_face_shield_khk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Face Shield (Khaki)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"_Tactical",
		"_Shades",
		"_ESS",
		"_Shemagh"
	};
	hiddenSelectionsTextures[] = 
	{
		"milgp_headgear\textures\f_face_shield_khk_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"_Tactical",
			"_Shades",
			"_ESS",
			"_Shemagh"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 4;
	};
};

class pca_nvg_face_shield_mc: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Face Shield (Multicam)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"_Tactical",
		"_Shades",
		"_ESS",
		"_Shemagh"
	};
	hiddenSelectionsTextures[] = 
	{
		"milgp_headgear\textures\f_face_shield_mc_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"_Tactical",
			"_Shades",
			"_ESS",
			"_Shemagh"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 4;
	};
};

class pca_nvg_face_shield_rgr: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Face Shield (Ranger Green)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"_Tactical",
		"_Shades",
		"_ESS",
		"_Shemagh"
	};
	hiddenSelectionsTextures[] = 
	{
		"milgp_headgear\textures\f_face_shield_rgr_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"_Tactical",
			"_Shades",
			"_ESS",
			"_Shemagh"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 4;
	};
};

class pca_nvg_face_shield_tan: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Face Shield (Tan)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"_Tactical",
		"_Shades",
		"_ESS",
		"_Shemagh"
	};
	hiddenSelectionsTextures[] = 
	{
		"milgp_headgear\textures\f_face_shield_tan_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"_Tactical",
			"_Shades",
			"_ESS",
			"_Shemagh"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 4;
	};
};

class pca_nvg_face_shield_shemagh_blk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Face Shield + Shemagh (Black)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"camo1",
		"_Tactical",
		"_Shades",
		"_ESS"
	};
	hiddenSelectionsTextures[] = 
	{
		"milgp_headgear\textures\f_face_shield_blk_co.paa",
		"milgp_headgear\textures\f_shemagh_blk_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"camo1",
			"_Tactical",
			"_Shades",
			"_ESS"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 8;
	};
};

class pca_nvg_face_shield_shemagh_cb: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Face Shield + Shemagh (Coyote Brown)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"camo1",
		"_Tactical",
		"_Shades",
		"_ESS"
	};
	hiddenSelectionsTextures[] = 
	{
		"milgp_headgear\textures\f_face_shield_cb_co.paa",
		"milgp_headgear\textures\f_shemagh_cb_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"camo1",
			"_Tactical",
			"_Shades",
			"_ESS"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 8;
	};
};

class pca_nvg_face_shield_shemagh_khk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Face Shield + Shemagh (Khaki)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"camo1",
		"_Tactical",
		"_Shades",
		"_ESS"
	};
	hiddenSelectionsTextures[] = 
	{
		"milgp_headgear\textures\f_face_shield_khk_co.paa",
		"milgp_headgear\textures\f_shemagh_khk_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"camo1",
			"_Tactical",
			"_Shades",
			"_ESS"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 8;
	};
};

class pca_nvg_face_shield_shemagh_mc: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Face Shield + Shemagh (Multicam)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"camo1",
		"_Tactical",
		"_Shades",
		"_ESS"
	};
	hiddenSelectionsTextures[] = 
	{
		"milgp_headgear\textures\f_face_shield_mc_co.paa",
		"milgp_headgear\textures\f_shemagh_rgr_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"camo1",
			"_Tactical",
			"_Shades",
			"_ESS"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 8;
	};
};

class pca_nvg_face_shield_shemagh_rgr: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Face Shield + Shemagh (Ranger Green)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"camo1",
		"_Tactical",
		"_Shades",
		"_ESS"
	};
	hiddenSelectionsTextures[] = 
	{
		"milgp_headgear\textures\f_face_shield_rgr_co.paa",
		"milgp_headgear\textures\f_shemagh_rgr_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"camo1",
			"_Tactical",
			"_Shades",
			"_ESS"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 8;
	};
};

class pca_nvg_face_shield_shemagh_tan: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Face Shield + Shemagh (Tan)";
	picture = "\rhsusf\addons\rhsusf_infantry2\ui\gogglesc_ca.paa";
	model = "\milgp_headgear\models\f_face_shield_base";
	hiddenSelections[] =
	{
		"camo",
		"camo1",
		"_Tactical",
		"_Shades",
		"_ESS"
	};
	hiddenSelectionsTextures[] = 
	{
		"milgp_headgear\textures\f_face_shield_tan_co.paa",
		"milgp_headgear\textures\f_shemagh_tan_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo",
			"camo1",
			"_Tactical",
			"_Shades",
			"_ESS"
		};
		uniformModel = "\milgp_headgear\models\f_face_shield_base";
		modelOff = "\milgp_headgear\models\f_face_shield_base";
		mass = 8;
	};
};

class pca_nvg_cigarette: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Cigarette";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_cigarette_ca.paa";
	model = "\a3_aegis\characters_f_aegis\facewear\cigarette";
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		uniformModel = "\a3_aegis\characters_f_aegis\facewear\cigarette";
		modelOff = "\a3_aegis\characters_f_aegis\facewear\cigarette";
		mass = 2;
	};
};

class pca_nvg_shemagh_lowered_cbr: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Coyote Brown)";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_shemag_tan_ca.paa";
	model = "\a3_aegis\characters_f_aegis\facewear\shemag";
	hiddenSelections[] =
	{
		"camo"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_aegis\facewear\data\shemag_cbr_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo"
		};
		uniformModel = "\a3_aegis\characters_f_aegis\facewear\shemag";
		modelOff = "\a3_aegis\characters_f_aegis\facewear\shemag";
		mass = 8;
	};
};

class pca_nvg_shemagh_lowered_khk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Khaki)";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_shemag_khk_ca.paa";
	model = "\a3_aegis\characters_f_aegis\facewear\shemag";
	hiddenSelections[] =
	{
		"camo"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_aegis\facewear\data\shemag_khk_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo"
		};
		uniformModel = "\a3_aegis\characters_f_aegis\facewear\shemag";
		modelOff = "\a3_aegis\characters_f_aegis\facewear\shemag";
		mass = 8;
	};
};

class pca_nvg_shemagh_lowered_oli: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Olive)";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_shemag_oli_ca.paa";
	model = "\a3_aegis\characters_f_aegis\facewear\shemag";
	hiddenSelections[] =
	{
		"camo"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_aegis\facewear\data\shemag_oli_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo"
		};
		uniformModel = "\a3_aegis\characters_f_aegis\facewear\shemag";
		modelOff = "\a3_aegis\characters_f_aegis\facewear\shemag";
		mass = 8;
	};
};

class pca_nvg_shemagh_lowered_red: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Red)";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_shemag_red_ca.paa";
	model = "\a3_aegis\characters_f_aegis\facewear\shemag";
	hiddenSelections[] =
	{
		"camo"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_aegis\facewear\data\shemag_red_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo"
		};
		uniformModel = "\a3_aegis\characters_f_aegis\facewear\shemag";
		modelOff = "\a3_aegis\characters_f_aegis\facewear\shemag";
		mass = 8;
	};
};

class pca_nvg_shemagh_lowered_tan: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Tan)";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_shemag_tan_ca.paa";
	model = "\a3_aegis\characters_f_aegis\facewear\shemag";
	hiddenSelections[] =
	{
		"camo"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_aegis\facewear\data\shemag_tan_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo"
		};
		uniformModel = "\a3_aegis\characters_f_aegis\facewear\shemag";
		modelOff = "\a3_aegis\characters_f_aegis\facewear\shemag";
		mass = 8;
	};
};

class pca_nvg_shemagh_lowered_white: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (White)";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_shemag_white_ca.paa";
	model = "\a3_aegis\characters_f_aegis\facewear\shemag";
	hiddenSelections[] =
	{
		"camo"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_aegis\facewear\data\shemag_white_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo"
		};
		uniformModel = "\a3_aegis\characters_f_aegis\facewear\shemag";
		modelOff = "\a3_aegis\characters_f_aegis\facewear\shemag";
		mass = 8;
	};
};

class pca_nvg_shemagh_loop_bge: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Loop, Beige)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\l_beige_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\l_beige_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_loop_blk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Loop, Black)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\l_black_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\l_black_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_loop_coy: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Loop, Coyote Brown)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\l_coyote_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\l_coyote_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_loop_gold: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Loop, Gold)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\l_gold_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\l_gold_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_loop_grn: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Loop, Green)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\l_green_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\l_green_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_loop_khk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Loop, Khaki)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\l_khaki_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\l_khaki_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_loop_tan: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Loop, Tan)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\l_tan_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\l_tan_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_loop_white: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Loop, White)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\l_white_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\l_white_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_mesh_bge: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Mesh, Beige)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\m_beige_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\m_beige_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_mesh_blk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Mesh, Black)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\m_black_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\m_black_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_mesh_coy: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Mesh, Coyote Brown)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\m_coyote_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\m_coyote_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_mesh_gold: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Mesh, Gold)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\m_gold_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\m_gold_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_mesh_grn: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Mesh, Green)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\m_green_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\m_green_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_mesh_khk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Mesh, Khaki)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\m_khaki_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\m_khaki_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_mesh_tan: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Mesh, Tan)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\m_tan_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\m_tan_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};

class pca_nvg_shemagh_mesh_white: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Mesh, White)";
	picture = "\a3\characters_f\data\ui\icon_g_combat_ca.paa";
	model = "\a3_aegis\characters_f_minos\data\fw\shemagh";
	hiddenSelections[]=
	{
		"camo2"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_minos\data\fw\m_white_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo2"
		};
		hiddenSelectionsTextures[] =
		{
			"\a3_aegis\characters_f_minos\data\fw\m_white_co.paa"
		};
		uniformModel = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		modelOff = "\a3_aegis\characters_f_minos\data\fw\shemagh";
		mass = 8;
	};
};