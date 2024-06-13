# geo_git
> A version control tools aiming for geometry shapes.
> With fancy GUIs.

# Releases
## Alpha 1.0
### Shapes
- Rectangles

### Features
- Basic Rectangle Operations
  - Create
  - Move
  - Delete
  - Differences between versions
- GUI
  - show all shapes, based on egui.
  - show differences between shapes.

# Protocol
GeoGit is based on scripts, which protocol is as follows:
```
AddRect $(Name) 124 122 144 135 $(OptionalImportance) $(OptionalHint) ;
AddRect $(Name) 1 2 3 4 $(OptionalImportance) $(OptionalHint) ;
UpdRect $(Name) 2 3 4 5 $(OptionalImportance) $(OptionalHint) ;
DelRect $(Name) ;
DelRect $(Name) ;
```
