"""
Constants defined in CODATA 2018.

Only the useful subset is defined.
Omitted are:
* multiple definitions of the same constant in different units
* values of units, including atomic and natural units
* ratios of other constants or units
"""

from ..constant import Constant
from .. import units as qu, prefixes as qp
from . import fundamental

# fmt: off

# CODATA 2018
alpha_particle_mass = Constant("m_α", "alpha_particle_mass", "6.6446573357e-27", qu.kg, "0.0000000020e-27", canon_symbol=False)
atomic_mass_constant = Constant("m_u", "atomic_mass_constant", "1.66053906660e-27", qu.kg, "0.00000000050e-27", canon_symbol=False)
Avogadro_constant = fundamental.Avogadro_constant
Bohr_magneton = Constant("μ_B", "Bohr_magneton", "9.2740100783e-24", qu.J * qu.T**-1, "0.0000000028e-24", canon_symbol=True)
Bohr_radius = Constant("a_0", "Bohr_radius", "5.29177210903e-11", qu.m, "0.00000000080e-11", canon_symbol=True)
Boltzmann_constant = fundamental.Boltzmann_constant
characteristic_impedance_of_vacuum = Constant(None, "characteristic_impedance_of_vacuum", "376.730313668", qu.ohm, "0.000000057", canon_symbol=False)
classical_electron_radius = Constant(None, "classical_electron_radius", "2.8179403262e-15", qu.m, "0.0000000013e-15", canon_symbol=False)
Compton_wavelength = Constant("λ", "Compton_wavelength", "2.42631023867e-12", qu.m, "0.00000000073e-12", canon_symbol=False)
#conductance_quantum = Constant(None, "conductance_quantum", "7.748091729...e-5", qu.S, 0), canon_symbol=False)
Copper_x_unit = Constant(None, "Copper_x_unit", "1.00207697e-13", qu.m, "0.00000028e-13", canon_symbol=False)
deuteron_g_factor = Constant("g_D", "deuteron_g_factor", "0.8574382338", qu.unitless, "0.0000000022", canon_symbol=False)
deuteron_mag_mom = Constant(None, "deuteron_mag_mom", "4.330735094e-27", qu.J * qu.T**-1, "0.000000011e-27", canon_symbol=False)
deuteron_mass = Constant("m_D", "deuteron_mass", "3.3435837724e-27", qu.kg, "0.0000000010e-27", canon_symbol=False)
deuteron_rms_charge_radius = Constant(None, "deuteron_rms_charge_radius", "2.12799e-15", qu.m, "0.00074e-15", canon_symbol=False)
electron_g_factor = Constant("g_e", "electron_g_factor", "-2.00231930436256", qu.unitless, "0.00000000000035", canon_symbol=False)
electron_gyromag_ratio = Constant("ɣ_e", "electron_gyromag_ratio", "1.76085963023e11", qu.s**-1 * qu.T**-1, "0.00000000053e11", canon_symbol=False)
electron_mag_mom = Constant(None, "electron_mag_mom", "-9.2847647043e-24", qu.J * qu.T**-1, "0.0000000028e-24", canon_symbol=False)
electron_mag_mom_anomaly = Constant(None, "electron_mag_mom_anomaly", "1.15965218128e-3", qu.unitless, "0.00000000018e-3", canon_symbol=False)
electron_mass = Constant("m_e", "electron_mass", "9.1093837015e-31", qu.kg, "0.0000000028e-31", canon_symbol=False)
elementary_charge = fundamental.elementary_charge
#Faraday_constant = Constant(None, "Faraday_constant", "96485.33212...", qu.C * qu.mol**-1, 0), canon_symbol=False)
Fermi_coupling_constant = Constant("G^0_F", "Fermi_coupling_constant", "1.1663787e-5", (qp.G * qu.eV)**-2, "0.0000006e-5", canon_symbol=False)
fine_structure_constant = Constant("α", "fine_structure_constant", "7.2973525693e-3", qu.unitless, "0.0000000011e-3", canon_symbol=True)
#first_radiation_constant = Constant(None, "first_radiation_constant", "3.741771852...e-16", qu.W * qu.m**2, 0), canon_symbol=False)
#first_radiation_constant_for_spectral_radiance = Constant(None, "first_radiation_constant_for_spectral_radiance", "1.191042972...e-16", qu.W * qu.m**2 * qu.sr**-1, 0), canon_symbol=False)
helion_g_factor = Constant(None, "helion_g_factor", "-4.255250615", qu.unitless, "0.000000050", canon_symbol=False)
helion_mag_mom = Constant(None, "helion_mag_mom", "-1.074617532e-26", qu.J * qu.T**-1, "0.000000013e-26", canon_symbol=False)
helion_mass = Constant(None, "helion_mass", "5.0064127796e-27", qu.kg, "0.0000000015e-27", canon_symbol=False)
helion_shielding_shift = Constant(None, "helion_shielding_shift", "5.996743e-5", qu.unitless, "0.000010e-5", canon_symbol=False)
hyperfine_transition_frequency_of_Cs_133 = fundamental.caesium_hyperfine_transition
#Josephson_constant = Constant(None, "Josephson_constant", "483597.8484...e9", qu.Hz * qu.V**-1, 0), canon_symbol=False)
lattice_parameter_of_silicon = Constant(None, "lattice_parameter_of_silicon", "5.431020511e-10", qu.m, "0.000000089e-10", canon_symbol=False)
lattice_spacing_of_ideal_Si_220 = Constant(None, "lattice_spacing_of_ideal_Si_220", "1.920155716e-10", qu.m, "0.000000032e-10", canon_symbol=False)
#Loschmidt_constant_27315_K_100_kPa = Constant(None, "Loschmidt_constant_27315_K_100_kPa", "2.651645804...e25", qu.m**-3, 0), canon_symbol=False)
#Loschmidt_constant_27315_K_101325_kPa = Constant(None, "Loschmidt_constant_27315_K_101325_kPa", "2.686780111...e25", qu.m**-3, 0), canon_symbol=False)
luminous_efficacy = fundamental.luminous_efficacy_540_THz
#mag_flux_quantum = Constant(None, "mag_flux_quantum", "2.067833848...e-15", qu.Wb, 0), canon_symbol=False)
molar_gas_constant = Constant("R", "molar_gas_constant", "8.31446261815324", qu.J * qu.mol**-1 * qu.K**-1, 0, alt_names=["gas_constant"], canon_symbol=True)
molar_mass_of_carbon_12 = Constant(None, "molar_mass_of_carbon_12", "11.9999999958e-3", qu.kg * qu.mol**-1, "0.0000000036e-3", canon_symbol=False)
molar_volume_of_silicon = Constant(None, "molar_volume_of_silicon", "1.205883199e-5", qu.m**3 * qu.mol**-1, "0.000000060e-5", canon_symbol=False)
Molybdenum_x_unit = Constant(None, "Molybdenum_x_unit", "1.00209952e-13", qu.m, "0.00000053e-13", canon_symbol=False)
muon_Compton_wavelength = Constant(None, "muon_Compton_wavelength", "1.173444110e-14", qu.m, "0.000000026e-14", canon_symbol=False)
muon_g_factor = Constant(None, "muon_g_factor", "-2.0023318418", qu.unitless, "0.0000000013", canon_symbol=False)
muon_mag_mom = Constant(None, "muon_mag_mom", "-4.49044830e-26", qu.J * qu.T**-1, "0.00000010e-26", canon_symbol=False)
muon_mag_mom_anomaly = Constant(None, "muon_mag_mom_anomaly", "1.16592089e-3", qu.unitless, "0.00000063e-3", canon_symbol=False)
muon_mass = Constant(None, "muon_mass", "1.883531627e-28", qu.kg, "0.000000042e-28", canon_symbol=False)
neutron_Compton_wavelength = Constant(None, "neutron_Compton_wavelength", "1.31959090581e-15", qu.m, "0.00000000075e-15", canon_symbol=False)
neutron_g_factor = Constant(None, "neutron_g_factor", "-3.82608545", qu.unitless, "0.00000090", canon_symbol=False)
neutron_gyromag_ratio = Constant(None, "neutron_gyromag_ratio", "1.83247171e8", qu.s**-1 * qu.T**-1, "0.00000043e8", canon_symbol=False)
neutron_mag_mom = Constant(None, "neutron_mag_mom", "-9.6623651e-27", qu.J * qu.T**-1, "0.0000023e-27", canon_symbol=False)
neutron_mass = Constant(None, "neutron_mass", "1.67492749804e-27", qu.kg, "0.00000000095e-27", canon_symbol=False)
neutron_to_shielded_proton_mag_mom_ratio = Constant(None, "neutron_to_shielded_proton_mag_mom_ratio", "-0.68499694", qu.unitless, "0.00000016", canon_symbol=False)
Newtonian_constant_of_gravitation = Constant(None, "Newtonian_constant_of_gravitation", "6.67430e-11", qu.m**3 * qu.kg**-1 * qu.s**-2, "0.00015e-11", canon_symbol=False)
nuclear_magneton = Constant(None, "nuclear_magneton", "5.0507837461e-27", qu.J * qu.T**-1, "0.0000000015e-27", canon_symbol=False)
Planck_constant = fundamental.Planck_constant
proton_Compton_wavelength = Constant(None, "proton_Compton_wavelength", "1.32140985539e-15", qu.m, "0.00000000040e-15", canon_symbol=False)
proton_g_factor = Constant(None, "proton_g_factor", "5.5856946893", qu.unitless, "0.0000000016", canon_symbol=False)
proton_gyromag_ratio = Constant(None, "proton_gyromag_ratio", "2.6752218744e8", qu.s**-1 * qu.T**-1, "0.0000000011e8", canon_symbol=False)
proton_mag_mom = Constant(None, "proton_mag_mom", "1.41060679736e-26", qu.J * qu.T**-1, "0.00000000060e-26", canon_symbol=False)
proton_mag_shielding_correction = Constant(None, "proton_mag_shielding_correction", "2.5689e-5", qu.unitless, "0.0011e-5", canon_symbol=False)
proton_mass = Constant(None, "proton_mass", "1.67262192369e-27", qu.kg, "0.00000000051e-27", canon_symbol=False)
proton_rms_charge_radius = Constant(None, "proton_rms_charge_radius", "8.414e-16", qu.m, "0.019e-16", canon_symbol=False)
quantum_of_circulation = Constant(None, "quantum_of_circulation", "3.6369475516e-4", qu.m**2 * qu.s**-1, "0.0000000011e-4", canon_symbol=False)
reduced_Compton_wavelength = Constant(None, "reduced_Compton_wavelength", "3.8615926796e-13", qu.m, "0.0000000012e-13", canon_symbol=False)
#reduced_Planck_constant = Constant(None, "reduced_Planck_constant", "1.054571817...e-34", qu.J * qu.s, 0), canon_symbol=False)
Rydberg_constant = Constant("R_∞", "Rydberg_constant", "10973731.568160", qu.m**-1, "0.000021", canon_symbol=True)
Sackur_Tetrode_constant_1_K_100_kPa = Constant(None, "Sackur_Tetrode_constant_1_K_100_kPa", "-1.15170753706", qu.unitless, "0.00000000045", canon_symbol=False)
Sackur_Tetrode_constant_1_K_101325_kPa = Constant(None, "Sackur_Tetrode_constant_1_K_101325_kPa", "-1.16487052358", qu.unitless, "0.00000000045", canon_symbol=False)
#second_radiation_constant = Constant(None, "second_radiation_constant", "1.438776877...e-2", qu.m * qu.K, 0), canon_symbol=False)
shielded_helion_gyromag_ratio = Constant(None, "shielded_helion_gyromag_ratio", "2.037894569e8", qu.s**-1 * qu.T**-1, "0.000000024e8", canon_symbol=False)
shielded_helion_mag_mom = Constant(None, "shielded_helion_mag_mom", "-1.074553090e-26", qu.J * qu.T**-1, "0.000000013e-26", canon_symbol=False)
shielded_proton_gyromag_ratio = Constant(None, "shielded_proton_gyromag_ratio", "2.675153151e8", qu.s**-1 * qu.T**-1, "0.000000029e8", canon_symbol=False)
shielded_proton_mag_mom = Constant(None, "shielded_proton_mag_mom", "1.410570560e-26", qu.J * qu.T**-1, "0.000000015e-26", canon_symbol=False)
speed_of_light_in_vacuum = fundamental.speed_of_light
standard_acceleration_of_gravity = Constant(None, "standard_acceleration_of_gravity", "9.80665", qu.m * qu.s**-2, 0, canon_symbol=False)
#Stefan_Boltzmann_constant = Constant(None, "Stefan_Boltzmann_constant", "5.670374419...e-8", qu.W * qu.m**-2 * qu.K**-4, 0), canon_symbol=False)
tau_Compton_wavelength = Constant(None, "tau_Compton_wavelength", "6.97771e-16", qu.m, "0.00047e-16", canon_symbol=False)
tau_mass = Constant(None, "tau_mass", "3.16754e-27", qu.kg, "0.00021e-27", canon_symbol=False)
Thomson_cross_section = Constant(None, "Thomson_cross_section", "6.6524587321e-29", qu.m**2, "0.0000000060e-29", canon_symbol=False)
triton_g_factor = Constant(None, "triton_g_factor", "5.957924931", qu.unitless, "0.000000012", canon_symbol=False)
triton_mag_mom = Constant(None, "triton_mag_mom", "1.5046095202e-26", qu.J * qu.T**-1, "0.0000000030e-26", canon_symbol=False)
triton_mass = Constant(None, "triton_mass", "5.0073567446e-27", qu.kg, "0.0000000015e-27", canon_symbol=False)
vacuum_electric_permittivity = Constant(None, "vacuum_electric_permittivity", "8.8541878128e-12", qu.F * qu.m**-1, "0.0000000013e-12", canon_symbol=False)
vacuum_mag_permeability = Constant(None, "vacuum_mag_permeability", "1.25663706212e-6", qu.N * qu.A**-2, "0.00000000019e-6", canon_symbol=False)
#von_Klitzing_constant = Constant(None, "von_Klitzing_constant", "25812.80745...", qu.ohm, 0), canon_symbol=False)
weak_mixing_angle = Constant(None, "weak_mixing_angle", "0.22290", qu.unitless, "0.00030", canon_symbol=False)
#Wien_frequency_displacement_law_constant = Constant(None, "Wien_frequency_displacement_law_constant", "5.878925757...e10", qu.Hz * qu.K**-1, 0), canon_symbol=False)
#Wien_wavelength_displacement_law_constant = Constant(None, "Wien_wavelength_displacement_law_constant", "2.897771955...e-3", qu.m * qu.K, 0), canon_symbol=False)

# fmt: on
