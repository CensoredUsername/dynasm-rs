initSidebarItems({"enum":[["RelocationKind","Specifies what kind of relocation a relocation is."],["RelocationSize","A descriptor for the size of a relocation. This also doubles as a relocation itself for relocations in data directives. Can be converted to relocations of any kind of architecture using `Relocation::from_size`."]],"struct":[["ImpossibleRelocation","Error returned when encoding a relocation failed"]],"trait":[["Relocation","Used to inform assemblers on how to implement relocations for each architecture. When implementing a new architecture, one simply has to implement this trait for the architecture’s relocation definition."]]});