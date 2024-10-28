import { Input, InputProps } from "@chakra-ui/react";

export default function CInput({ ...props }: InputProps) {
  return (
    <Input
      bg={"neutral.100"}
      color={"neutral.600"}
      py={6}
      rounded={"xl"}
      border={"none"}
      {...props}
    />
  )
}