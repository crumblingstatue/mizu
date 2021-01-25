gb_tests!(
    brk; // clock until break

    dmg_acid2_test,
    "dmg-acid2.gb",
    13523824884037480967;

    // since we are not running the CGB boot_rom/bios, the colors when using
    // DMG or CGB emulation is the same
    dmg_acid2_test_dmg for "dmg",
    "dmg-acid2.gb",
    13523824884037480967;

    cgb_acid2_test,
    "cgb-acid2.gbc",
    4378550468433865064;
);
