import {NgModule} from '@angular/core';
import {BrowserModule} from '@angular/platform-browser';
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {HTTP_INTERCEPTORS, HttpClientModule} from '@angular/common/http';

import {AppComponent} from './app.component';
import {AppRoutingModule} from './routes/app-routing.module';
import {APIInterceptor} from './api.interceptor';
import {MessagesComponent} from './components/messages/messages.component';
import {PostCardComponent} from './components/post-card/post-card.component';
import {PostListComponent} from './components/post-list/post-list.component';
import {PostComponent} from './components/post/post.component';
import {HeaderComponent} from './components/header/header.component';
import {FooterComponent} from './components/footer/footer.component';
import {LoginComponent} from './components/login/login.component';
import {PostEditComponent} from './components/post-edit/post-edit.component';
import {MaterialModule} from "./material/material.module";
import {DeletePostDialogComponent} from './components/delete-post-dialog/delete-post-dialog.component';


@NgModule({
  declarations: [
    AppComponent,
    MessagesComponent,
    PostCardComponent,
    PostListComponent,
    PostComponent,
    HeaderComponent,
    FooterComponent,
    LoginComponent,
    PostEditComponent,
    DeletePostDialogComponent
  ],
  imports: [
    BrowserModule,
    FormsModule,
    ReactiveFormsModule,
    AppRoutingModule,
    HttpClientModule,
    MaterialModule
  ],
  providers: [{
    provide: HTTP_INTERCEPTORS,
    useClass: APIInterceptor,
    multi: true,
  }],
  bootstrap: [AppComponent]
})
export class AppModule {
}
