import {TestBed} from '@angular/core/testing';

import {APIInterceptorInterceptor} from './apiinterceptor.interceptor';

describe('APIInterceptorInterceptor', () => {
  beforeEach(() => TestBed.configureTestingModule({
    providers: [
      APIInterceptorInterceptor
    ]
  }));

  it('should be created', () => {
    const interceptor: APIInterceptorInterceptor = TestBed.inject(APIInterceptorInterceptor);
    expect(interceptor).toBeTruthy();
  });
});
