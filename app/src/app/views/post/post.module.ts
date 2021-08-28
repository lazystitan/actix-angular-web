import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import {PostRoutingModule} from './post-routing.module';
import {PostComponent} from '../../components/post/post.component';
import {MaterialModule} from '../../material/material.module';
import {DeletePostDialogComponent} from '../../components/delete-post-dialog/delete-post-dialog.component';



@NgModule({
  declarations: [
    PostComponent,
    DeletePostDialogComponent
  ],
  imports: [
    CommonModule,
    PostRoutingModule,
    MaterialModule
  ]
})
export class PostModule { }
