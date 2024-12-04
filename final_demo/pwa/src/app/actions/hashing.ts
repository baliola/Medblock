"use server"

import bcrypt from "bcrypt";

const SALTED_ROUNDS = 10;

export interface ResponseHashPin {
  data?: string;
  error?: string;
  message: string;
}

export const hashPin = async ({
  pin,
  principal
}: {
  pin: string;
  principal: string;
}) => {
  try {
    const merged = `${principal}@${pin}`;
    const data = await bcrypt.hash(merged, SALTED_ROUNDS); 
  
    return JSON.stringify({
      data: data,
      message: "Pin hashed successfully"
    })
  } catch (error) {
    return JSON.stringify({
      error: error,
      message: "Error hashing pin"
    })
  }
}

export const comparePin = async({
  pin,
  principal,
  hash
}: {
  pin: string;
  principal: string;
  hash: string;
}) => {
  console.log(pin, principal, hash);
  try {
    const merged = `${principal}@${pin}`;
    const data = await bcrypt.compare(merged, hash);
  
    if (!data) {
      throw new Error("Pin does not match")
    }
    
    return JSON.stringify({
      data: data,
      message: "Pin compared successfully"
    })
  } catch (error) {
    return JSON.stringify({
      error: error,
      message: "Error comparing pin"
    })
  }
}