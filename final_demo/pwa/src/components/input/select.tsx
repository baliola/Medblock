import { Input, InputProps, Select, SelectProps } from "@chakra-ui/react";

export default function CSelect({ children, ...props }: SelectProps) {
  return (
    <Select
      bg={"neutral.100"}
      color={"neutral.600"}
      h={12}
      rounded={"xl"}
      border={"none"}
      {...props}
    >
      {children}
    </Select>
  )
}