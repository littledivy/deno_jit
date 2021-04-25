`jit` from `js`

Execute raw machine code from JavaScript. I hope you know what you're doing :)

```typescript
const inst = new Uint8Array([0xC3]); // ret
const noop = jit(inst);
noop();
```

MIT License
