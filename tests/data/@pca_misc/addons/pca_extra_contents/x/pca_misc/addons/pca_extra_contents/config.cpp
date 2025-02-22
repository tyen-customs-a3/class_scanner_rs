class CfgPatches
{
	class pca_misc_pca_extra_contents
	{
		name="Extra Contents";
		author="PCA";
		units[]={};
		weapons[]={};
		requiredAddons[]=
		{
			"pca_misc_pca_main",
			"rhs_c_troops",
			"rhsusf_c_troops"
		};
		requiredVersion=1.6;
	};
};

#include "CfgVehicles.hpp"
#include "CfgWeapons.hpp"