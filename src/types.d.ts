declare namespace WebAssembly {
  function instantiateStreaming(
    promise: Promise<any>,
    importObject?: any,
  ): Promise<WebAssemblyInstantiatedSource>;
}
