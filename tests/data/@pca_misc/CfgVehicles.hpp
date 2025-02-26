class CfgVehicles 
{
	class Bag_Base;
	class pca_backpack_invisible: Bag_Base 
	{
		author = "PCA";
		scope = 2;
		displayName = "Invisible Backpack";
		model = "\a3\weapons_f\empty";
		picture = "";
		maximumLoad = 240;
		mass = 20;
	};
	class pca_backpack_invisible_large: Bag_Base 
	{
		author = "PCA";
		scope = 2;
		displayName = "Invisible Backpack (Large)";
		model = "\a3\weapons_f\empty";
		picture = "";
		maximumLoad = 320;
		mass = 40;
	};
	
	class pca_backpack_taki_jacket_base: Bag_Base 
	{
		hiddenSelections[]=
		{
			"camo1",
			"camo2"
		};
	};
	class pca_backpack_taki_jacket1_01: pca_backpack_taki_jacket_base 
	{
		author = "PCA";
		scope = 2;
		displayName = "Takistani Jacket (Chocolate Chip)";
		model = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\cup_taki_jacket.p3d";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket_01_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor01_1_co.paa"
		};
		maximumLoad = 320;
		mass = 40;
	};
	class pca_backpack_taki_jacket1_02: pca_backpack_taki_jacket1_01 
	{
		displayName = "Takistani Jacket (Snow)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket_02_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor01_2_co.paa"
		};
	};
	class pca_backpack_taki_jacket1_03: pca_backpack_taki_jacket1_01 
	{
		displayName = "Takistani Jacket (Woodland)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket_03_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor01_3_co.paa"
		};
	};
	class pca_backpack_taki_jacket1_04: pca_backpack_taki_jacket1_01 
	{
		displayName = "Takistani Jacket (Black)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket_04_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind01_1_co.paa"
		};
	};
	class pca_backpack_taki_jacket1_05: pca_backpack_taki_jacket1_01 
	{
		displayName = "Takistani Jacket (Grey)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket_05_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind01_2_co.paa"
		};
	};
	class pca_backpack_taki_jacket1_06: pca_backpack_taki_jacket1_01 
	{
		displayName = "Takistani Jacket (Brown)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket_06_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind01_3_co.paa"
		};
	};
	class pca_backpack_taki_jacket2_01: pca_backpack_taki_jacket_base 
	{
		author = "PCA";
		scope = 2;
		displayName = "Takistani Waistcoat w/ rigs (Woodland)";
		model = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\cup_taki_jacket2.p3d";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket2_01_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor05_1_co.paa"
		};
		maximumLoad = 320;
		mass = 40;
	};
	class pca_backpack_taki_jacket2_02: pca_backpack_taki_jacket2_01 
	{
		displayName = "Takistani Waistcoat w/ rigs (Faded Woodland)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket2_02_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor05_2_co.paa"
		};
	};
	class pca_backpack_taki_jacket2_03: pca_backpack_taki_jacket2_01 
	{
		displayName = "Takistani Waistcoat w/ rigs (Desert)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket2_03_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor05_3_co.paa"
		};
	};
	class pca_backpack_taki_jacket2_04: pca_backpack_taki_jacket2_01 
	{
		displayName = "Takistani Waistcoat w/ rigs (Black)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket2_04_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind05_1_co.paa"
		};
	};
	class pca_backpack_taki_jacket2_05: pca_backpack_taki_jacket2_01 
	{
		displayName = "Takistani Waistcoat w/ rigs (Grey)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket2_05_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind05_2_co.paa"
		};
	};
	class pca_backpack_taki_jacket2_06: pca_backpack_taki_jacket2_01 
	{
		displayName = "Takistani Waistcoat w/ rigs (Brown)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket2_06_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind05_3_co.paa"
		};
	};
	class pca_backpack_taki_jacket3_01: pca_backpack_taki_jacket_base 
	{
		author = "PCA";
		scope = 2;
		displayName = "Takistani Utility Jacket (Woodland)";
		model = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\cup_taki_jacket3.p3d";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket3_01_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor02_1_co.paa"
		};
		maximumLoad = 320;
		mass = 40;
	};
	class pca_backpack_taki_jacket3_02: pca_backpack_taki_jacket3_01 
	{
		displayName = "Takistani Utility Jacket (Chocolate Chip)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket3_02_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor02_2_co.paa"
		};
	};
	class pca_backpack_taki_jacket3_03: pca_backpack_taki_jacket3_01 
	{
		displayName = "Takistani Utility Jacket (Desert)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket3_03_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor02_3_co.paa"
		};
	};
	class pca_backpack_taki_jacket3_04: pca_backpack_taki_jacket3_01 
	{
		displayName = "Takistani Utility Jacket (Black)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket3_04_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind02_1_co.paa"
		};
	};
	class pca_backpack_taki_jacket3_05: pca_backpack_taki_jacket3_01 
	{
		displayName = "Takistani Utility Jacket (Grey)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket3_05_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind02_2_co.paa"
		};
	};
	class pca_backpack_taki_jacket3_06: pca_backpack_taki_jacket3_01 
	{
		displayName = "Takistani Utility Jacket (Brown)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket3_06_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind02_3_co.paa"
		};
	};
	class pca_backpack_taki_jacket4_01: pca_backpack_taki_jacket_base 
	{
		author = "PCA";
		scope = 2;
		displayName = "Takistani Jacket w/ rigs (Woodland)";
		model = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\cup_taki_jacket4.p3d";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket4_01_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor03_1_co.paa"
		};
		maximumLoad = 320;
		mass = 40;
	};
	class pca_backpack_taki_jacket4_02: pca_backpack_taki_jacket4_01 
	{
		displayName = "Takistani Jacket w/ rigs (Desert)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket4_02_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor03_2_co.paa"
		};
	};
	class pca_backpack_taki_jacket4_03: pca_backpack_taki_jacket4_01 
	{
		displayName = "Takistani Jacket w/ rigs (Tri-Color Desert)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket4_03_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor03_3_co.paa"
		};
	};
	class pca_backpack_taki_jacket4_04: pca_backpack_taki_jacket4_01 
	{
		displayName = "Takistani Jacket w/ rigs (Black)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket4_04_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind03_1_co.paa"
		};
	};
	class pca_backpack_taki_jacket4_05: pca_backpack_taki_jacket4_01 
	{
		displayName = "Takistani Jacket w/ rigs (Grey)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket4_05_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind03_2_co.paa"
		};
	};
	class pca_backpack_taki_jacket4_06: pca_backpack_taki_jacket4_01 
	{
		displayName = "Takistani Jacket w/ rigs (Brown)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket4_06_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind03_3_co.paa"
		};
	};
	class pca_backpack_taki_jacket5_01: pca_backpack_taki_jacket_base 
	{
		author = "PCA";
		scope = 2;
		displayName = "Takistani Light Jacket (Woodland)";
		model = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\cup_taki_jacket5.p3d";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket5_01_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor05_1_co.paa"
		};
		maximumLoad = 320;
		mass = 40;
	};
	class pca_backpack_taki_jacket5_02: pca_backpack_taki_jacket5_01 
	{
		displayName = "Takistani Light Jacket (Desert)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket5_02_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor05_2_co.paa"
		};
	};
	class pca_backpack_taki_jacket5_03: pca_backpack_taki_jacket5_01 
	{
		displayName = "Takistani Light Jacket (Tri-Color Desert)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket5_03_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor05_3_co.paa"
		};
	};
	class pca_backpack_taki_jacket5_04: pca_backpack_taki_jacket5_01 
	{
		displayName = "Takistani Light Jacket (Blue/Grey)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket5_04_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind05_1_co.paa"
		};
	};
	class pca_backpack_taki_jacket5_05: pca_backpack_taki_jacket5_01 
	{
		displayName = "Takistani Light Jacket (Brown)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket5_05_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind05_2_co.paa"
		};
	};
	class pca_backpack_taki_jacket5_06: pca_backpack_taki_jacket5_01 
	{
		displayName = "Takistani Light Jacket (Light Brown)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket5_06_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind05_3_co.paa"
		};
	};
	class pca_backpack_taki_jacket6_01: pca_backpack_taki_jacket_base 
	{
		author = "PCA";
		scope = 2;
		displayName = "Takistani Waistcoat (Woodland)";
		model = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\cup_taki_jacket6.p3d";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket6_01_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor02_1_co.paa"
		};
		maximumLoad = 320;
		mass = 40;
	};
	class pca_backpack_taki_jacket6_02: pca_backpack_taki_jacket6_01 
	{
		displayName = "Takistani Waistcoat (Desert)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket6_02_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor02_2_co.paa"
		};
	};
	class pca_backpack_taki_jacket6_03: pca_backpack_taki_jacket6_01 
	{
		displayName = "Takistani Waistcoat (Tri-Color Desert)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket5_03_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_opfor02_3_co.paa"
		};
	};
	class pca_backpack_taki_jacket6_04: pca_backpack_taki_jacket6_01 
	{
		displayName = "Takistani Waistcoat (Blue/Gray)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket5_04_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind02_1_co.paa"
		};
	};
	class pca_backpack_taki_jacket6_05: pca_backpack_taki_jacket6_01 
	{
		displayName = "Takistani Waistcoat (Brown)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket5_05_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind02_2_co.paa"
		};
	};
	class pca_backpack_taki_jacket6_06: pca_backpack_taki_jacket6_01 
	{
		displayName = "Takistani Waistcoat (Light Brown)";
		picture = "\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\ui\icon_v_jacket5_06_ca.paa";
		hiddenSelectionsTextures[] =
		{
			"\cup\creatures\people\military\cup_creatures_people_military_takiinsurgents\data\loc_ind02_3_co.paa"
		};
	};
};