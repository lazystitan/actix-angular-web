import { Component, OnInit } from '@angular/core';
import {TokenStorageService} from "../../service/auth/token-storage.service";

@Component({
  selector: 'app-header',
  templateUrl: './header.component.html',
  styleUrls: ['./header.component.scss']
})
export class HeaderComponent implements OnInit {
  title = 'Riton\'s blog';

  isLogin = false;

  constructor(
    private tokenService: TokenStorageService
  ) { }

  ngOnInit(): void {
    this.tokenService.isLogin.subscribe((value) => {
      this.isLogin = value;
    })
  }

  signOut(): void {
    this.tokenService.signOut();
  }

}
