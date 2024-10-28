import { FormLabel, FormLabelProps } from "@chakra-ui/react";

export default function FormCLabel({ children, ...props }: FormLabelProps) {
  return (
    <FormLabel
      fontSize={'sm'}
      color={"neutral.700"}
      fontWeight={'normal'}
      {...props}
    >
      {children}
    </FormLabel>
  )
}