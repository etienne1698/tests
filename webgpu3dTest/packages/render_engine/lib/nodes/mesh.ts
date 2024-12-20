import { mat4, vec2 } from "gl-matrix";
import { Node3D } from "./node_3d";
import { Geometry } from "../models/geometry";
import { Material } from "../models/material";

export type MeshOptions = {
  geometry: Geometry;
  material: Material;
  transform?: mat4;
};

export class Mesh extends Node3D {
  geometry!: Geometry;
  material!: Material;

  constructor(options: MeshOptions) {
    super();
    Object.assign(this, options);
  }
}
