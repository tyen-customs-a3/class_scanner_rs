class baseMan 
{
	displayName = "Unarmed";
	// All Randomized
	uniform[] = {};
	vest[] = {};
	backpack[] = {};
	headgear[] = {};
	goggles[] = {};
	hmd[] = {};
	// Leave empty to not change faces and Insignias
	faces[] = {};
	insignias[] = {};
	// All Randomized. Add Primary Weapon and attachments
	// Leave Empty to remove all. {"Default"} for using original items the character start with
	primaryWeapon[] = {};
	scope[] = {};
	bipod[] = {};
	attachment[] = {};
	silencer[] = {};
	// SecondaryAttachments[] arrays are NOT randomized
	secondaryWeapon[] = {};
	secondaryAttachments[] = {};
	sidearmWeapon[] = {};
	sidearmAttachments[] = {};
	// These are added to the uniform or vest first - overflow goes to backpack if there's any
	magazines[] = {};
	items[] = {};
	// These are added directly into their respective slots
	linkedItems[] = 
	{
		"ItemWatch",
		"ItemMap",
		"ItemCompass"
	};
	// These are put directly into the backpack
	backpackItems[] = {};
	// This is executed after the unit init is complete. Argument: _this = _unit
	code = "";
};

class rm : baseMan
{
	displayName = "Rifleman";
	uniform[] = 
	{
		LIST_1("U_BG_Guerrilla_6_1"),
		LIST_1("U_BG_Guerilla1_1"),
		LIST_1("U_BG_Guerilla2_2"),
		LIST_1("U_BG_Guerilla2_1"),
		LIST_1("U_BG_Guerilla2_3"),
		LIST_1("U_BG_leader"),
		LIST_1("aegis_guerilla_garb_m81"),
		LIST_1("aegis_sweater_grn"),
		LIST_1("aegis_guerilla_tshirt_m81"),
		LIST_1("aegis_guerilla_tshirt_m81_alt")
	};
	vest[] = 
	{
		LIST_1("milgp_v_mmac_assaulter_belt_cb"),
		LIST_1("milgp_v_mmac_assaulter_belt_rgr"),
		LIST_1("milgp_v_mmac_assaulter_belt_khk")
	};
	headgear[] = 
	{
		LIST_1("pca_headband_blk"),
		LIST_1("pca_headband"),
		LIST_1("pca_headband_red"),
		LIST_1("pca_headband_tan")
	};
	backpack[] = 
	{
		LIST_1("wsx_tacticalpack_oli"),
		LIST_1("aegis_tacticalpack_cbr"),
		LIST_1("rhs_rk_sht_30_olive"),
		LIST_1("rhssaf_kitbag_md2camo")
	};
	goggles[] = 
	{
		LIST_1("G_Bandanna_blk"),
		LIST_1("G_Bandanna_khk"),
		LIST_1("G_Bandanna_oli"),
		LIST_1("bear_bandana_m81")
	};
	primaryWeapon[] = 
	{
		"sfp_weap_ak5c_blk"
	};
	scope[] = 
	{
		"optic_mrco"
	};
	items[] =
	{
		"ACRE_PRC343",
		LIST_10("ACE_fieldDressing"),
		LIST_5("ACE_packingBandage"),
		LIST_5("ACE_quikclot"),
		LIST_4("ACE_tourniquet"),
		LIST_2("ACE_epinephrine"),
		LIST_2("ACE_morphine"),
		LIST_2("ACE_splint")
	};
	magazines[] = 
	{
		LIST_2("SmokeShell"),
		LIST_2("rhs_mag_m67"),
		LIST_13("pca_mag_30Rnd_556x45_M855A1_PMAG_Blk")
	};
	backpackItems[] = 
	{
		LIST_4("pca_mag_30Rnd_556x45_M855A1_PMAG_Blk")
	};
};

class ar : rm 
{
	displayName = "Automatic Rifleman";
	primaryWeapon[] = 
	{
		"sps_weap_kac_lamg_hg_blk"
	};
	sidearmWeapon[] = 
	{
		"aegis_weap_fnx45_blk"
	};
	vest[] = 
	{
		LIST_1("milgp_v_mmac_hgunner_belt_cb"),
		LIST_1("milgp_v_mmac_hgunner_belt_rgr"),
		LIST_1("milgp_v_mmac_hgunner_belt_khk")
	};
	backpack[] = 
	{
		"B_Carryall_cbr"
	};
	bipod[] = 
	{
		"CUP_bipod_VLTOR_Modpod_black"
	};
	magazines[] = 
	{
		LIST_2("SmokeShell"),
		LIST_2("rhs_mag_m67"),
		LIST_2("11Rnd_45ACP_Mag"),
		LIST_3("sps_200Rnd_556x45_M855A1_Mixed_KAC_Box")
	};
	backpackItems[] = 
	{
		LIST_4("sps_200Rnd_556x45_M855A1_Mixed_KAC_Box")
	};
};
