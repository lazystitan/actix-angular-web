import {Injectable, NgModule} from '@angular/core';
import {
  ActivatedRouteSnapshot,
  CanActivate,
  Router,
  RouterModule,
  RouterStateSnapshot,
  Routes,
  UrlTree
} from '@angular/router';
import {PostComponent} from "../components/post/post.component";
import {PostListComponent} from "../components/post-list/post-list.component";
import {LoginComponent} from "../components/login/login.component";
import {PostEditComponent} from "../components/post-edit/post-edit.component";
import {Observable} from "rxjs";
import {TokenStorageService} from "../service/auth/token-storage.service";

@Injectable()
class CanActivateTeam implements CanActivate {
  constructor(private router: Router, private tokenService: TokenStorageService) {}

  canActivate(
    route: ActivatedRouteSnapshot,
    state: RouterStateSnapshot
  ): Observable<boolean|UrlTree>|Promise<boolean|UrlTree>|boolean|UrlTree {
    if (!this.tokenService.isLogin()) {
      this.router.navigate(['/login']);
      return false;
    }
    return true;
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
  providers: [CanActivateTeam]

})
export class AppRoutingModule {
}
