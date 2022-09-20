import * as BufferLayout from '@solana/buffer-layout';
import { Buffer } from 'buffer';

//buffer is a byte array
export const createIncrementInstruction = (): Buffer => {
  const layout = BufferLayout.struct([BufferLayout.u8('instruction')]);
  const data = Buffer.alloc(layout.span);
  layout.encode({ instruction: 0 }, data);
  return data;
};

export const createDecrementInstruction = (): Buffer => {
  const layout = BufferLayout.struct([BufferLayout.u8('instruction')]);
  const data = Buffer.alloc(layout.span);
  layout.encode({ instruction: 1 }, data);
  return data;
};

export const createSetInstruction = (value: number): Buffer => {
  const layout = BufferLayout.struct([
    BufferLayout.u8('instruction'),
    BufferLayout.u32('value'),
  ]);
  const data = Buffer.alloc(layout.span);
  layout.encode({ instruction: 2, value: value }, data);
  return data;
};
