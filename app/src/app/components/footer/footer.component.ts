import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-footer',
  templateUrl: './footer.component.html',
  styleUrls: ['./footer.component.scss']
})
export class FooterComponent implements OnInit {

  email = "ritonelion@outlook.com"
  github_url = "//github.com/lazystitan"
  record_url = "http://beian.miit.gov.cn/"

  constructor() { }

  ngOnInit(): void {
  }

}
