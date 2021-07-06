import {Component, OnInit} from '@angular/core';
import {NavigationEnd, Router} from "@angular/router";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {

  isPostsPage = false;

  constructor(private router: Router) {
  }

  ngOnInit(): void {
    let self = this;
    this.router.events.subscribe({
      next(e) {
        if (e instanceof NavigationEnd) {
          self.isPostsPage = ['/posts', '/'].indexOf(e.url) !== -1;
        }
      }
    })
  }
}
