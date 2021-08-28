import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {PostRoutingModule} from './post-routing.module';
import {PostComponent, SafeHtmlPipe} from '../../components/post/post.component';
import {MaterialModule} from '../../material/material.module';
import {DeletePostDialogComponent} from '../../components/delete-post-dialog/delete-post-dialog.component';


@NgModule({
  declarations: [
    PostComponent,
    DeletePostDialogComponent,
    SafeHtmlPipe
  ],
  imports: [
    CommonModule,
    PostRoutingModule,
    MaterialModule
  ]
})
export class PostModule {
}
