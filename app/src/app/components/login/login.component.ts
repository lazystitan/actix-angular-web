import {Component, OnInit} from '@angular/core';
import {FormBuilder, FormGroup, Validators} from '@angular/forms';
import {AuthService} from '../../service/auth/auth.service';
import {TokenStorageService} from '../../service/auth/token-storage.service';
import {Router} from '@angular/router';


@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss']
})
export class LoginComponent implements OnInit {
  hide = true;
  loginForm: FormGroup;
  errorTips: { [key: string]: string } = {
    username: '',
    password: ''
  };
  validationMessage: { [key: string]: { [key: string]: string } } = {
    username: {
      required: '请输入用户名',
      minlength: '用户名过短'
    },
    password: {
      required: '请输入密码',
      minlength: '密码过短'
    }
  };

  constructor(
    private formBuilder: FormBuilder,
    private authService: AuthService,
    private tokenStorageService: TokenStorageService,
    private router: Router
  ) {
    this.loginForm = this.formBuilder.group({
      username: ['', [Validators.required, Validators.minLength(3)]],
      password: ['', [Validators.required, Validators.minLength(6)]]
    });
    this.loginForm.valueChanges.subscribe(data => this.onValueChanged(data));
  }

  ngOnInit(): void {
  }

  onSubmit(userData: FormGroup): void {
    if (this.validLoginForm()) {
      this.authService.login(this.loginForm.get('username')?.value, this.loginForm.get('password')?.value)
        .subscribe(data => {
          if (data.code === 0) {
            this.tokenStorageService.saveToken(data.token);
            this.router.navigate(['/post_edit']);
          } else {
            alert(data.message);
            this.loginForm.reset();
          }
        });
    }
  }

  validLoginForm(afterChange: boolean = false): boolean {
    let haveError = false;
    for (const field in Object.keys(this.errorTips)) {
      this.errorTips[field] = '';
      const ctl = this.loginForm.get(field);
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
            if (ctl.errors.hasOwnProperty(key)) {
              this.errorTips[field] += message[key] + '\n';
              haveError = true;
            }
          }
        }
      }
    }
    return !haveError;
  }


  onValueChanged(data: FormGroup): void {
    this.validLoginForm(true);
  }

}
