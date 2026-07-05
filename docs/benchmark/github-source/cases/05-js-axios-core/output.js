'use strict';

import utils from './../utils.js';
import buildURL from '../helpers/buildURL.js';
import InterceptorManager from './InterceptorManager.js';
import dispatchRequest from './dispatchRequest.js';
import mergeConfig from './mergeConfig.js';
import buildFullPath from './buildFullPath.js';
import validator from '../helpers/validator.js';
import AxiosHeaders from './AxiosHeaders.js';

const validators = validator.validators;

/**
 * Create a new instance of Axios
 *
 * @param {Object} instanceConfig The default config for the instance
 *
 * @return {Axios} A new instance of Axios
 */
class Axios {
  constructor(instanceConfig) { … 7 line(s) … ⟦tj:85ff9c5c66feb1b491f41a975e6a6f9d⟧ }

  /**
   * Dispatch a request
   *
   * @param {String|Object} configOrUrl The config specific for this request (merged with this.defaults)
   * @param {?Object} config
   *
   * @returns {Promise} The Promise to be fulfilled
   */
  async request(configOrUrl, config) { … 26 line(s) … ⟦tj:73549c6b288c6a9fac50042748655afd⟧ }

  _request(configOrUrl, config) { … 122 line(s) … ⟦tj:93b647e9468d9b26d0aadd3d6e52c0ec⟧ }

  getUri(config) { … 5 line(s) … ⟦tj:af84becd19d51a4aad2068449d395381⟧ }
}

// Provide aliases for supported request methods
utils.forEach(['delete', 'get', 'head', 'options'], function forEachMethodNoData(method) {
  /*eslint func-names:0*/
  Axios.prototype[method] = function(url, config) {
    return this.request(mergeConfig(config || {}, {
      method,
      url,
      data: (config || {}).data
    }));
  };
});

utils.forEach(['post', 'put', 'patch'], function forEachMethodWithData(method) {
  /*eslint func-names:0*/

  function generateHTTPMethod(isForm) { … 12 line(s) … ⟦tj:e2268cc2d236a9175d0c670cdc20873f⟧ }

  Axios.prototype[method] = generateHTTPMethod();

  Axios.prototype[method + 'Form'] = generateHTTPMethod(true);
});

export default Axios;
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (6383 bytes) is available by calling tinyjuice_retrieve with token "bac65bab7ca6cf2c0815dda20bf490bd" (marker ⟦tj:bac65bab7ca6cf2c0815dda20bf490bd⟧)]