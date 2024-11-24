# geo_git
> A version control tools aiming for geometry shapes.
> With fancy GUIs.

# Releases
## Alpha
### Shapes
- Rectangles

### Features
- Basic Rectangle Operations
  - Create
  - Move
  - Delete
  - Differences between versions (x)
- GUI
  - show all shapes, based on egui.
  - show differences between shapes.

# Protocol
See an example in `test/formal.txt`.

[Commit ... ] See `Commit`

## Commit
```
Commit
[ Action ... ]
```

### Action
```
  <actiontype> <shape> [color] [gradient] ;
```

### Actiontype
Actiontype is an ENUM which is
```
Add | Del | Upd
```
### Shape
```
<shapetype> <geom>
```
#### ShapeType
`shapetype` is a enum 
```
Rect Line
```
#### Geom
Each `geom` has its own 

### Geom
  f