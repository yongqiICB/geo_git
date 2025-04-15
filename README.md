# geo_git
A fast Rectangle Visualizer for massive objects

## QuickStart
The project aims to make people get know of the place of different objects from plain txt.

For example, you have a vlsi design with thousands of cells. You have a `.def` file but, however, do not know what it looks like. 
Geogit can quickly help you get a quick, obvious understanding of it. And you can know the history of each cells! Cool! 
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
