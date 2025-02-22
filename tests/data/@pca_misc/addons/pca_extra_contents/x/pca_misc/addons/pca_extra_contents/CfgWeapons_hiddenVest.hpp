class pca_vest_invisible: Vest_Camo_Base 
{
	author = "PCA";
	scope = 2;
	displayName = "Invisible Vest";
	model = "\a3\weapons_f\empty";
	
	class ItemInfo: ItemInfo
	{
		uniformModel = "\a3\weapons_f\empty";
		containerClass = Supply200;		
		mass = 20;
		
		class HitpointsProtectionInfo
		{
			class Chest
			{
				hitPointName = "HitChest";
				armor = 4;
				passThrough = 0.5;
			};
			class Diaphragm
			{
				hitPointName = "HitDiaphragm";
				armor = 4;
				passThrough = 0.5;
			};
			class Abdomen
			{
				hitPointName = "HitAbdomen";
				armor = 4;
				passThrough = 0.5;
			};
			class Pelvis
			{
				hitPointName = "HitPelvis";
				armor = 4;
				passThrough = 0.5;
			};
			class Body
			{
				hitPointName = "HitBody";
				passThrough = 0.5;
			};
		};
	};
};

class pca_vest_invisible_kevlar: pca_vest_invisible 
{
	author = "PCA";
	scope = 2;
	displayName = "Invisible Vest (Kevlar)";
	
	class ItemInfo: ItemInfo
	{
		mass = 40;
		
		class HitpointsProtectionInfo
		{
			class Chest
			{
				hitPointName = "HitChest";
				armor = 12;
				passThrough = 0.4;
			};
			class Diaphragm
			{
				hitPointName = "HitDiaphragm";
				armor = 12;
				passThrough = 0.4;
			};
			class Abdomen
			{
				hitPointName = "HitAbdomen";
				armor = 12;
				passThrough = 0.4;
			};
			class Pelvis
			{
				hitPointName = "HitPelvis";
				armor = 12;
				passThrough = 0.4;
			};
			class Body
			{
				hitPointName = "HitBody";
				passThrough = 0.4;
			};
		};
	};
};

class pca_vest_invisible_plate: pca_vest_invisible 
{
	author = "PCA";
	scope = 2;
	displayName = "Invisible Vest (Plate)";
	
	class ItemInfo: ItemInfo
	{
		mass = 80;
		
		class HitpointsProtectionInfo
		{
			class Chest
			{
				hitPointName = "HitChest";
				armor = 24;
				passThrough = 0.2;
			};
			class Diaphragm
			{
				hitPointName = "HitDiaphragm";
				armor = 24;
				passThrough = 0.2;
			};
			class Abdomen
			{
				hitPointName = "HitAbdomen";
				armor = 24;
				passThrough = 0.2;
			};
			class Pelvis
			{
				hitPointName = "HitPelvis";
				armor = 24;
				passThrough = 0.2;
			};
			class Body
			{
				hitPointName = "HitBody";
				passThrough = 0.2;
			};
		};
	};
};