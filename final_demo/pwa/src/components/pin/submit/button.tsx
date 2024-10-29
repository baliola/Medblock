"use client"

import { Button, Flex, useToast } from "@chakra-ui/react";
import { usePinStore } from "@/store/pin-store";
import { PIN_LENGTH } from "@/components/pin/input";
import { useUserPrincipal } from "@ic-reactor/react";
import { comparePin, ResponseHashPin } from "@/app/actions/hashing";
import { buttonPin } from "@/constants/contents/pin/button";

interface AddPinButtonProps {
  onClose: () => void;
  onSuccess: () => void;
}

export default function SubmitPinButton({
  onClose,
  onSuccess
}: AddPinButtonProps) {
  const toast = useToast();

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
      const response = await comparePin({
        pin: pin,
        principal: hexPrincipal,
        hash: hash
      });
      const parsedResponse = JSON.parse(response) as ResponseHashPin;

      if (parsedResponse.data) {
        setPin("");
        onSuccess();

        return toast({
          title: "Success Entered Pin",
          description: "You can now proceed",
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
          description: "Pin does not match!",
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
        onClick={() => {
          setPin("");
          onClose();
        }}
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