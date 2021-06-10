import {Component, OnInit} from '@angular/core';
import {FormBuilder, FormGroup, Validators} from "@angular/forms";


@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.less']
})
export class LoginComponent implements OnInit {
  loginForm: FormGroup;
  errorTips: { [key: string]: string } = {
    name: '',
    password: ''
  };
  validationMessage: { [key: string]: { [key: string]: string } } = {
    name: {
      'required': '请输入用户名',
      'minlength': '用户名过短'
    },
    password: {
      'required': '请输入密码',
      'minlength': '密码过短'
    }
  };

  constructor(
    private formBuilder: FormBuilder
  ) {
    this.loginForm = this.formBuilder.group({
      'name': ['', [Validators.required, Validators.minLength(3)]],
      'password': ['', [Validators.required, Validators.minLength(6)]]
    })
    this.loginForm.valueChanges.subscribe(data => this.onValueChanged(data))
  }

  ngOnInit(): void {
  }

  onSubmit(userData: FormGroup) {
    if (this.validLoginForm()) {
      this.loginForm.reset();
    }
  }

  validLoginForm(afterChange : boolean = false) {
    let haveError = false;
    for (const field in this.errorTips) {
      this.errorTips[field] = '';
      const ctl = this.loginForm.get(field)
      if (ctl) {
        let check: any = false;
        if (afterChange) {
          check = (!afterChange || ctl.dirty) && !ctl.valid;
        } else {
          check = !ctl.valid;
        }
        if (check) {
          const message = this.validationMessage[field];
          for (const key in ctl.errors) {
            this.errorTips[field] += message[key] + '\n';
            haveError = true;
          }
        }
      }
    }
    return !haveError
  }


  onValueChanged(_data: { name: string, password: string }) {
    this.validLoginForm(true)
  }

}
