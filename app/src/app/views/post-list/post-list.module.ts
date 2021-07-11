import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import {PostListRoutingModule} from "./post-list-routing.module";
import {PostCardComponent} from '../../components/post-card/post-card.component';
import {PostListComponent} from '../../components/post-list/post-list.component';
import {MaterialModule} from "../../material/material.module";



@NgModule({
  declarations: [
    PostCardComponent,
    PostListComponent
  ],
  imports: [
    CommonModule,
    PostListRoutingModule,
    MaterialModule
  ]
})
export class PostListModule { }
