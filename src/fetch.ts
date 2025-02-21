import { invoke } from "@tauri-apps/api/core";

export type Method = 'GET' | 'POST';

class FPRequestInit {
    method: Method;
    header: object | null;
    body: object | null;
    constructor(method: Method, header?: object, body?: object) {
        this.method = method;
        this.header = header || null;
        this.body = body || null;
    }
}

export function FPFetch(url: string, init: FPRequestInit) {
    return invoke('fetch', {
        url: url,
        init: init
    })
}

/* Example

    #POST
    FPFetch("https://httpbin.org/post", {
      method: "POST",
      header: {
        "quote-token": "73d72ca11e"
      },
      body: {
        "name": "Finper"
      }
    }).then(value => {
      console.log(JSON.parse(value.body));
    })

    #GET
    FPFetch("https://httpbin.org/get?key1=value1&key2=value2", {
      method: "GET",
      header: {
        "quote-token": "73d72ca11e"
      },
    }).then(value => {
      console.log(JSON.parse(value.body));
    })

*/
