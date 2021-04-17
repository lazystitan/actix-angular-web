import {Injectable} from '@angular/core';
import {HttpEvent, HttpHandler, HttpInterceptor, HttpRequest} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from "../environments/environment";

@Injectable()
export class APIInterceptor implements HttpInterceptor {

  constructor() {
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

    const apiReq = request.clone({url: `${protocal}//${backenUrl}${port}/apis/${request.url}`});
    return next.handle(apiReq);
  }
}
