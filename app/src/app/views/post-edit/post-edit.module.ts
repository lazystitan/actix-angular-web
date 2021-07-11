import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import {PostEditRoutingModule} from "./post-edit-routing.module";
import {PostEditComponent} from "../../components/post-edit/post-edit.component";
import {MaterialModule} from "../../material/material.module";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";



@NgModule({
  declarations: [
    PostEditComponent,
  ],
  imports: [
    CommonModule,
    MaterialModule,
    PostEditRoutingModule,
    FormsModule,
    ReactiveFormsModule,
  ]
})
export class PostEditModule { }
