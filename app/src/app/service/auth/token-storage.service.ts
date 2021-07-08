import { Injectable } from '@angular/core';
import {BehaviorSubject} from "rxjs";

const TOKEN_KEY = 'auth-token'

@Injectable({
  providedIn: 'root'
})
export class TokenStorageService {

  isLogin = new BehaviorSubject(false);

  constructor() {
    if (this.getToken()) {
      this.isLogin.next(true);
    }
  }

  signOut(): void {
    window.sessionStorage.clear();
    this.isLogin.next(false);
    // window.location.reload();
  }

  saveToken(token: string): void {
    window.sessionStorage.removeItem(TOKEN_KEY);
    window.sessionStorage.setItem(TOKEN_KEY, token);
    this.isLogin.next(true);
  }

  getToken(): string | null {
    return window.sessionStorage.getItem(TOKEN_KEY);
  }
}
