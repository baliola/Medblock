"use client"

import { useRef, useState } from "react";
import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { Button, Flex, Input, Text, useToast } from "@chakra-ui/react";

import { ClaimConsentRequest, ClaimConsentResponse } from "@/declarations/patient_registry/patient_registry.did";
import { usePatientMethod } from "@/services/patients";
import { usePatientStore } from "@/store/patient-management";

const OTP_LENGTH = 6;

export default function InputConcentCode() {
  const toast = useToast();
  const params = useSearchParams();
  const pathname = usePathname();
  const router = useRouter();

  const addPatient = usePatientStore(state => state.addPatient);

  const onShowPatient = (session_id: string) => {
    const param = new URLSearchParams(params);
    param.delete('concent_input');
    param.append('id', session_id);

    const newUrl = `${pathname}?${param.toString()}`;
    router.push(newUrl);
  }

  const inputsRef = useRef<(HTMLInputElement | null)[]>([]);
  const [otp, setOtp] = useState<string[]>(Array(OTP_LENGTH).fill(""));

  const {
    call: sendConcentCode,
    loading: loadingSendConcentCode,
  } = usePatientMethod({
    functionName: "claim_consent",
    onSuccess(data) {
      setOtp(
        Array(OTP_LENGTH).fill("")
      );

      if (!data) return null;

      const {
        name,
        session_id
      }: ClaimConsentResponse = data;

      addPatient({
        name: name,
        session_id: session_id
      });

      toast({
        title: "Concent Claimed!",
        description: "You have access to the EMR user!",
        status: "success",
      });

      onShowPatient(session_id)
      return;
    },
    refetchOnMount: false,
    onError(error) {
      setOtp(
        Array(OTP_LENGTH).fill("")
      );

      if (
        error?.message &&
        error.message
          .toLowerCase()
          .includes("consent already claimed or does not exists")
      ) {
        toast({
          title: "Error Claim Concent Code!",
          description: "It looks like concent code already claimed or does not exists",
          status: "error",
        });

        return;
      } else {
        toast({
          title: "Error Claim Concent Code!",
          description: "Something went wrong please try again later!",
          status: "error",
        });

        return;
      }
    },
  });

  const onChange = (
    event: React.ChangeEvent<HTMLInputElement>,
    index: number
  ) => {
    const { value } = event.target;

    if (
      /^\d*$/.test(value) &&
      value.length <= 1
    ) {
      const newOtp = [...otp];
      newOtp[index] = value;
      setOtp(newOtp);

      if (value !== "" && index < otp.length - 1) {
        inputsRef.current[index + 1]?.focus();
      }
    }
  };

  const onSubmit = async () => {
    const concentCode: string = otp.join("");
    const data: ClaimConsentRequest = {
      code: concentCode
    };

    // @ts-ignore
    await sendConcentCode([data])
  }

  const isOtpValid = otp.every((digit) => digit !== "");

  return (
    <Flex direction="column" align="center" gap={5} pt={5}>
      <Text fontSize={{ base: "sm" }} textAlign="center" color="neutral.700">
        Enter Consent Code
      </Text>
      <Flex align="center" gap={2}>
        {otp.map((value, index) => (
          <Input
            key={index}
            type="number"
            onChange={(event) => onChange(event, index)}
            ref={(el) => { inputsRef.current[index] = el }}
            value={value}
            bg="primary.200"
            rounded="lg"
            textAlign="center"
            fontSize="sm"
            fontWeight="semibold"
            placeholder="*"
            px={0} py={4}
          />
        ))}
      </Flex>
      <Button
        colorScheme="primary"
        bg="primary.700"
        w="full"
        rounded="xl"
        fontSize={'xs'}
        onClick={onSubmit}
        isDisabled={!isOtpValid}
        isLoading={loadingSendConcentCode}
      >
        Access
      </Button>
    </Flex>
  )
}