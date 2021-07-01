import {Injectable} from '@angular/core';
import {HttpEvent, HttpHandler, HttpHeaders, HttpInterceptor, HttpRequest} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from "../environments/environment";
import {TokenStorageService} from "./service/auth/token-storage.service";

const TOKEN_HEADER_KEY = 'Authorization'

@Injectable()
export class APIInterceptor implements HttpInterceptor {

  constructor(private token: TokenStorageService) {
  }

  intercept(request: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    let protocal = document.location.protocol;
    let port = '';
    if (protocal == 'https:' && !environment.production) {
      port = ':8083';
    } else if (protocal == 'http:' && !environment.production) {
      port = ':8080';
    }
    let backenUrl = environment.backenUrl;

    const token = this.token.getToken();

    let params: {
      headers: HttpHeaders | undefined;
      url: string
    } = {
      headers: undefined,
      url: `${protocal}//${backenUrl}${port}/apis/${request.url}`
    }

    if (token != null) {
      params.headers = request.headers.set(TOKEN_HEADER_KEY, token)
    }
    let apiReq = request.clone(params);

    return next.handle(apiReq);
  }
}
