import { Component, OnInit } from '@angular/core';
import {FormBuilder, FormGroup} from "@angular/forms";

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.less']
})
export class LoginComponent implements OnInit {

  loginForm : FormGroup;

  constructor(
    private formBuilder : FormBuilder
  ) {
    this.loginForm = this.formBuilder.group({
      'name' : '',
      'password' : ''
    })
  }

  ngOnInit(): void {
  }

  onSubmit(userData : FormGroup) {
    console.log(userData);
    this.loginForm.reset();
  }

}
