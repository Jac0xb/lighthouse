import { useId } from 'react'

export function Logo(props) {
  const id = useId()

  return (
    <svg
      width="112"
      height="112"
      viewBox="0 0 112 112"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      {...props}
    >
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M112 10C112 4.47715 107.523 0 102 0H10C4.47715 0 0 4.47715 0 10V52C0 57.5228 4.47715 62 10 62H14.5C18.6421 62 22 65.3579 22 69.5V69.5C22 73.6421 18.6421 77 14.5 77H10C4.47715 77 0 81.4772 0 87V102C0 107.523 4.47715 112 10 112H21.2133C23.8655 112 26.409 110.946 28.2844 109.071L49.2844 88.0711C53.1896 84.1658 59.5213 84.1658 63.4265 88.0711L84.4265 109.071C86.3019 110.946 88.8454 112 91.4976 112H102C107.523 112 112 107.523 112 102V87C112 81.4772 107.523 77 102 77H97.5C93.3579 77 90 73.6421 90 69.5V69.5C90 65.3579 93.3579 62 97.5 62H102C107.523 62 112 57.5228 112 52V10Z"
        fill={`url(#${id}-gradient)`}
      />
      <defs>
        <linearGradient
          id={`${id}-gradient`}
          x1="56"
          y1="0"
          x2="56"
          y2="112"
          gradientUnits="userSpaceOnUse"
        >
          <stop stopColor="#F0ABFC" />
          <stop offset="1" stopColor="#E546E9" />
        </linearGradient>
      </defs>
    </svg>
  )
}
