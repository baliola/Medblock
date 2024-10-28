"use client"

import { useRef, useState } from "react";
import { Flex, Input } from "@chakra-ui/react";
import { usePinStore } from "@/store/pin-store";

export const PIN_LENGTH = 6;

export default function InputPIN() {
  const inputsRef = useRef<(HTMLInputElement | null)[]>([]);
  const setPin = usePinStore(state => state.setPin);
  const [tempPin, setTempPin] = useState<string[]>(Array(PIN_LENGTH).fill(""));

  const onChange = (
    event: React.ChangeEvent<HTMLInputElement>,
    index: number
  ) => {
    const { value } = event.target;

    if (
      /^\d*$/.test(value) &&
      value.length <= 1
    ) {
      const newOtp = [...tempPin];
      newOtp[index] = value;

      setTempPin(newOtp);
      setPin(newOtp.join(""));

      if (value !== "" && index < tempPin.length - 1) {
        inputsRef.current[index + 1]?.focus();
      }
    }
  };

  return (
    <Flex align="center" gap={3}>
      {tempPin.map((value, index) => (
        <Input
          key={index}
          type="number"
          onChange={(event) => onChange(event, index)}
          ref={(el) => { inputsRef.current[index] = el }}
          value={value}
          bg="primary.200"
          rounded="xl"
          textAlign="center"
          fontSize="xl"
          fontWeight="semibold"
          placeholder="*"
          h={20}
        />
      ))}
    </Flex>
  )
}