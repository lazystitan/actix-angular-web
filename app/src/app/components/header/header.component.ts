import { Component, OnInit } from '@angular/core';
import {TokenStorageService} from "../../service/auth/token-storage.service";

@Component({
  selector: 'app-header',
  templateUrl: './header.component.html',
  styleUrls: ['./header.component.scss']
})
export class HeaderComponent implements OnInit {
  title = 'Riton\'s blog';

  constructor(
    public tokenService: TokenStorageService
  ) { }

  ngOnInit(): void {
  }

}
