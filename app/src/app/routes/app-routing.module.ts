import {Injectable, NgModule} from '@angular/core';
import {ActivatedRouteSnapshot, CanActivate, RouterModule, RouterStateSnapshot, Routes, UrlTree} from '@angular/router';
import {PostComponent} from "../component/post/post.component";
import {PostListComponent} from "../component/post-list/post-list.component";
import {LoginComponent} from "../component/login/login.component";
import {PostEditComponent} from "../component/post-edit/post-edit.component";
import {Observable} from "rxjs";

class UserToken {}

class Permissions {
  canActivate(user: UserToken, id: string): boolean {
    console.log(UserToken)
    console.log(id)
    return true;
  }
}

@Injectable()
class CanActivateTeam implements CanActivate {
  constructor(private permissions: Permissions, private currentUser: UserToken) {}

  canActivate(
    route: ActivatedRouteSnapshot,
    state: RouterStateSnapshot
  ): Observable<boolean|UrlTree>|Promise<boolean|UrlTree>|boolean|UrlTree {
    return this.permissions.canActivate(this.currentUser, route.params.id);
  }
}

const routes: Routes = [
  {path: '', redirectTo: '/posts', pathMatch: 'full'},
  {path: 'posts', component: PostListComponent},
  {path: 'post/:id', component: PostComponent},
  {path: 'login', component: LoginComponent},
  {path: 'post_edit', component: PostEditComponent, canActivate: [CanActivateTeam]}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
  providers: [CanActivateTeam, UserToken, Permissions]

})
export class AppRoutingModule {
}
