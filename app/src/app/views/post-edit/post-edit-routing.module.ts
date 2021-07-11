import { NgModule } from '@angular/core';
import {RouterModule, Routes} from "@angular/router";
import {PostEditComponent} from "../../components/post-edit/post-edit.component";

const routes: Routes = [
  {
    path: '',
    component: PostEditComponent
  }
]

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class PostEditRoutingModule { }
