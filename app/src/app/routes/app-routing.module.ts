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
import {Observable} from 'rxjs';
import {TokenStorageService} from '../service/auth/token-storage.service';

@Injectable()
class CanActivateTeam implements CanActivate {
  constructor(private router: Router, private tokenService: TokenStorageService) {
  }

  canActivate(
    route: ActivatedRouteSnapshot,
    state: RouterStateSnapshot
  ): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {
    if (!this.tokenService.isLogin.value) {
      this.router.navigate(['/login']);
      return false;
    }
    return true;
  }
}

const routes: Routes = [
  {path: '', redirectTo: '/posts', pathMatch: 'full'},
  {
    path: 'posts',
    loadChildren: () => import('../views/post-list/post-list.module').then(m => m.PostListModule)
  },
  {
    path: 'post/:id',
    loadChildren: () => import('../views/post/post.module').then(m => m.PostModule)
  },
  {
    path: 'login',
    loadChildren: () => import('../views/login/login.module').then(m => m.LoginModule)
  },
  {
    path: 'post_edit/:id',
    loadChildren: () => import('../views/post-edit/post-edit.module').then(m => m.PostEditModule),
    canActivate: [CanActivateTeam]
  },
  {
    path: 'post_edit',
    loadChildren: () => import('../views/post-edit/post-edit.module').then(m => m.PostEditModule),
    canActivate: [CanActivateTeam]
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
  providers: [CanActivateTeam]

})
export class AppRoutingModule {
}
