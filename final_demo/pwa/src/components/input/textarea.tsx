import { Textarea, TextareaProps } from "@chakra-ui/react";

export default function CTextArea({ ...props }: TextareaProps) {
  return (
    <Textarea
      bg={"neutral.100"}
      color={"neutral.600"}
      rounded={"xl"}
      border={"none"}
      minH={28}
      {...props}
    />
  )
}