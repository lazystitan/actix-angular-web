import { Injectable } from '@angular/core';

const TOKEN_KEY = 'auth-token'

@Injectable({
  providedIn: 'root'
})
export class TokenStorageService {

  _isLogin = false;

  constructor() { }

  signOut(): void {
    window.sessionStorage.clear();
    this._isLogin = false;
    window.location.reload();
  }

  saveToken(token: string): void {
    window.sessionStorage.removeItem(TOKEN_KEY);
    window.sessionStorage.setItem(TOKEN_KEY, token);
    this._isLogin = true;
  }

  getToken(): string | null {
    return window.sessionStorage.getItem(TOKEN_KEY);
  }

  isLogin(): Boolean {
    if (this.getToken() !== null ) {
      this._isLogin = true;
    }
    return this._isLogin;
  }
}
