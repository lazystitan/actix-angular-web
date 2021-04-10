import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-footer',
  templateUrl: './footer.component.html',
  styleUrls: ['./footer.component.less']
})
export class FooterComponent implements OnInit {

  email = "ritonelion@outlook.com"
  github_url = "//github.com/lazystitan"

  constructor() { }

  ngOnInit(): void {
  }

}
