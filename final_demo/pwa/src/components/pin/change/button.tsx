"use client"

import { useRouter } from "next/navigation";
import { Button, Flex, useToast } from "@chakra-ui/react";
import { usePinStore } from "@/store/pin-store";
import { PIN_LENGTH } from "@/components/pin/input";
import { useUserPrincipal } from "@ic-reactor/react";
import { hashPin, ResponseHashPin } from "@/app/actions/hashing";
import { buttonPin } from "@/constants/contents/pin/button";

export default function ChangePinButton() {
  const toast = useToast();
  const router = useRouter();

  const pin = usePinStore(state => state.pin);
  const setPin = usePinStore(state => state.setPin);

  const principal = useUserPrincipal();

  const onSubmit = async () => {
    if (!principal) return;

    const hexPrincipal = principal.toHex();
    const textPrincipal = principal.toText();

    const hash = localStorage.getItem(`pin@${textPrincipal}`);

    if (!hash) return;

    try {
      const response = await hashPin({
        pin: pin,
        principal: hexPrincipal
      });
      const parsedResponse = JSON.parse(response) as ResponseHashPin;

      if (parsedResponse.data) {
        setPin("");

        localStorage.setItem(`pin@${textPrincipal}`, parsedResponse.data);
        router.replace("/setting");

        return toast({
          title: "Pin Changed!",
          description: "You have successfully changed your pin!",
          isClosable: true,
          duration: 5000,
          status: "success",
          position: "top-right",
        })
      }

      if (parsedResponse.error) {
        console.log(parsedResponse.error);
        return toast({
          title: "Error!",
          description: "Something went wrong!",
          isClosable: true,
          duration: 5000,
          status: "error",
          position: "top-right",
        });
      }
    } catch (error) {
      return toast({
        title: "Error!",
        description: "Something went wrong!",
        isClosable: true,
        duration: 5000,
        status: "error",
        position: "top-right",
      });
    }
  }

  return (
    <Flex direction={'column'} gap={3}>
      <Button
        onClick={() => router.push(buttonPin.cancel.redirect)}
      >
        {buttonPin.cancel.label}
      </Button>
      <Button
        colorScheme="primary"
        bg={"primary.700"}
        isDisabled={pin.length !== PIN_LENGTH}
        onClick={onSubmit}
      >
        {buttonPin.submit.label}
      </Button>
    </Flex>
  )
}